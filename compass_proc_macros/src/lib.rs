extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LibrusSingular)]
pub fn librus_singular(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named, .. }), .. }) = input.data {
        named
    } else {
        panic!("Only structs with named fields are supported")
    };

    let ftype = &fields[0].ty;
    let fname = &fields[0].ident.as_ref().unwrap();

    let expanded = quote! {
        impl LibrusTypeSingular<#ftype> for #name {
            fn get(&self) -> &#ftype {
                &self.#fname
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(LibrusPlural)]
pub fn librus_plural(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named, .. }), .. }) = input.data {
        named
    } else {
        panic!("Only structs with named fields are supported")
    };

    // Get type in Vec<T> generic
    let ftype = if let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) = &fields[0].ty {
        if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &segments[0].arguments {
            if let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. })) = &args[0] {
                segments[0].ident.clone()
            } else {
                panic!("Only structs with named fields are supported")
            }
        } else {
            panic!("Only structs with named fields are supported")
        }
    } else {
        panic!("Only structs with named fields are supported")
    };

    let fname = &fields[0].ident.as_ref().unwrap();

    let expanded = quote! {
        impl LibrusTypePlural<#ftype> for #name {
            fn get(&self) -> &Vec<#ftype> {
                &self.#fname
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn librus_structs(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as syn::Ident);

    let sing_name = format!("{}Response", name);
    let sing_name = syn::Ident::new(&sing_name, name.span());
    let pl_name = format!("{}sResponse", name);
    let pl_name = syn::Ident::new(&pl_name, name.span());

    let name_without_api = name.to_string().replace("API", "");
    let plural_name_without_api = format!("{}s", name_without_api);

    let name_without_api_lower = name_without_api.to_lowercase();
    let plural_name_without_api_lower = format!("{}s", name_without_api_lower);

    // let name_without_api = syn::Ident::new(&name_without_api, name.span());
    // let plural_name_without_api = syn::Ident::new(&plural_name_without_api, name.span());

    let name_without_api_lower = syn::Ident::new(&name_without_api_lower, name.span());
    let plural_name_without_api_lower = syn::Ident::new(&plural_name_without_api_lower, name.span());


    let expanded = quote! {
        #[derive(LibrusSingular, Serialize, Deserialize)]
        pub struct #sing_name {
            #[serde(alias = #name_without_api)]
            pub #name_without_api_lower: #name,
        }

        #[derive(LibrusPlural, Serialize, Deserialize)]
        pub struct #pl_name {
            #[serde(alias = #plural_name_without_api)]
            pub #plural_name_without_api_lower: Vec<#name>,
        }
    };

    TokenStream::from(expanded)
}