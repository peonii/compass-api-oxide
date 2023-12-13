use std::{sync::Arc, collections::HashMap};
use reqwest::header::HeaderMap;

use anyhow::Result;
use serde::{Serialize, Deserialize};

use super::api::LibrusResource;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

pub struct LibrusClient {
    pub token: Option<String>,
    pub req: reqwest::Client,
}

#[derive(Serialize, Deserialize)]
pub struct LibrusCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct APISynergiaAccountsWrapper {
    pub accounts: Vec<APISynergiaAccount>,
}

#[derive(Serialize, Deserialize)]
pub struct APISynergiaAccount {
    pub id: i32,
    #[serde(alias = "accessToken")]
    pub access_token: String,
    pub login: String,
}

impl LibrusClient {
    pub fn new() -> Self {
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::default();
        let cookie_store = Arc::new(cookie_store);

        Self {
            token: None,
            req: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .cookie_store(true)
                .cookie_provider(cookie_store)
                .build()
                .unwrap(),
        }
    }

    async fn get_csrf(&self) -> Result<String> {
        let response = self.req.get("https://portal.librus.pl/").send().await?;

        // Find csrf token

        let response_text = response.text().await?;

        // Search for regex: /<meta name="csrf-token" content="(.*)">/g
        let re = regex::Regex::new(r#"<meta name="csrf-token" content="(.*)">"#)?;

        let csrf = re
            .captures(&response_text)
            .ok_or(anyhow::anyhow!("Couldn't fetch the CSRF token!"))?
            .get(1)
            .ok_or(anyhow::anyhow!("Couldn't fetch the CSRF token!"))?
            .as_str();

        Ok(csrf.to_string())
    }

    pub async fn log_in(&mut self, email: String, password: String) -> Result<()> {
        let mut headers = HeaderMap::new();

        let csrf = self.get_csrf().await?;

        headers.insert("X-CSRF-Token", csrf.parse()?);
        headers.insert("User-Agent", USER_AGENT.parse()?);
        headers.insert("Content-Type", "application/json".parse()?);

        let credentials = LibrusCredentials { email, password };

        let response_cookies = self
            .req
            .post("https://portal.librus.pl/konto-librus/login/action")
            .headers(headers)
            .json(&credentials)
            .send()
            .await?;

        if response_cookies.status() != 200 {
            return Err(anyhow::anyhow!("Invalid credentials!"));
        }

        let response = self
            .req
            .get("https://portal.librus.pl/api/v3/SynergiaAccounts")
            .send()
            .await?;

        
        let accounts: APISynergiaAccountsWrapper = response.json::<APISynergiaAccountsWrapper>().await?;

        if accounts.accounts.is_empty() {
            return Err(anyhow::anyhow!("No accounts found!"));
        }

        self.token = Some(accounts.accounts[0].access_token.clone());

        Ok(())
    }

    pub async fn request<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut header = HeaderMap::new();

        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in!"))?;

        header.insert("Authorization", format!("Bearer {}", token).parse()?);
        header.insert("gzip", "true".parse()?);

        let response = self
            .req
            .get(url)
            .headers(header)
            .send()
            .await?;

        let response = response.json::<T>().await?;

        Ok(response)
    }

    pub async fn fetch_resources<T>(&self, resources: Vec<LibrusResource>, one: &str, many: &str) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        if resources.is_empty() {
            return Err(anyhow::anyhow!("No resources provided!"));
        }

        // Cut "https://api.librus.pl/<ver>/" from the url of the first resource
        let url = &resources[0].url[26..];
        // Replace all occurences of the resource's id with ""
        let url = url.replace(&resources[0].id.to_string(), "");

        let mut url = format!("https://api.librus.pl/3.0/{}", url);

        // Add the ids of the resources to the url
        for resource in resources.iter() {
            url += resource.id.to_string().as_str();
        }

        let mut results: Vec<T> = vec![];

        if resources.len() == 1 {
            let response = self.request::<HashMap<String, T>>(&url).await?;
            let response = response.get(one).ok_or(anyhow::anyhow!("Invalid resource!"))?;

            results.push(response.clone());
        } else {
            let response = self.request::<HashMap<String, Vec<T>>>(&url).await?;
            let response = response.get(many).ok_or(anyhow::anyhow!("Invalid resource!"))?;

            for result in response {
                results.push(result.clone());
            }
        }

        Ok(results)
    }
}