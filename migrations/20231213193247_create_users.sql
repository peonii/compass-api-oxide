-- Add migration script here
create table users (
    id uuid primary key default gen_random_uuid(),
    email text not null unique,
    password text not null,
    librus_access_token text not null,
    next_check_at timestamptz not null,
    is_test_account boolean not null default false,
    first_name text not null,
    last_name text not null,
    created_at timestamptz not null default now()
);