use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde::Deserialize;

use crate::JsonObject;

#[derive(Debug, Clone)]
pub struct AuthClient {
    pub headers: Option<HeaderMap>,
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct SupabaseUser {
    id: String,
    aud: String,
    role: String,
    email: String,
    email_confirmed_at: String,
    phone: String,
    confirmation_sent_at: String,
    confirmed_at: String,
    last_sign_in_at: String,
    app_metadata: JsonObject,
    user_metadata: JsonObject,
    identities: JsonObject,
    created_at: String,
    updated_at: String,
    is_anonymous: bool
}

impl AuthClient {
    pub fn new(url: &str, headers: Option<HeaderMap>) -> Self {
        Self {
            url: url.to_owned(),
            headers
        }
    }

    pub async fn get_user(&self) -> Result<SupabaseUser, Box<dyn std::error::Error>> {
        let client = reqwest::ClientBuilder::new();

        let client = if let Some(headers) = &self.headers {
            client.default_headers(headers.clone()).build()
        } else {
            client.build()
        }?;

        let response = client.get(format!("{}/user", self.url))
            .send().await?;

        let user: SupabaseUser = response.json().await?;

        Ok(user)
    }
}
