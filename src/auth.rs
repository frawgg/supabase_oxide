use std::collections::HashMap;

use anyhow::anyhow;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use base64::prelude::*;

use crate::JsonObject;

#[derive(Debug, Clone)]
pub struct AuthClient {
    pub headers: Option<HeaderMap>,
    pub url: String,
    apikey: String,
    bearer: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct SupabaseUser {
    pub id: String,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub email_confirmed_at: String,
    pub phone: String,
    pub confirmation_sent_at: String,
    pub confirmed_at: String,
    pub last_sign_in_at: String,
    pub app_metadata: JsonObject,
    pub user_metadata: JsonObject,
    pub identities: Vec<JsonObject>,
    pub created_at: String,
    pub updated_at: String,
    pub is_anonymous: bool
}

impl AuthClient {
    pub fn new(url: &str, headers: Option<HeaderMap>, apikey: String, bearer: Option<String>) -> Self {
        Self {
            url: url.to_owned(),
            headers,
            apikey,
            bearer
        }
    }

    pub async fn get_user(&self) -> Result<SupabaseUser, anyhow::Error> {
        let client = reqwest::Client::new();

        let response = client.get(format!("{}/user", self.url))
            .header("Authorization", self.bearer.clone().unwrap_or("".to_owned()))
            .header("apikey", &self.apikey)
            .send().await?.text().await?;
        println!("{:#?}", response);

        let user: SupabaseUser = serde_json::from_str(&response)?;

        Ok(user)
    }

    pub async fn get_claims(&self) -> Result<HashMap<String, serde_json::Value>, anyhow::Error> {
        // Ensure user's JWT is valid
        let _ = self.get_user().await?;

        let parts: Vec<String> = if let Some(bearer) = &self.bearer {
            bearer.replace("Bearer ", "").split(".").map(|s| s.to_owned()).collect()
        } else {
            return Err(anyhow!("No JWT"))
        };

        let bytes = if let Some(part) = parts.get(1) {
            BASE64_STANDARD_NO_PAD.decode(part)?
        } else {
            return Err(anyhow!("Invalid JWT structure"))
        };
        let claims_str = String::from_utf8(bytes)?;
        let claims: HashMap<String, serde_json::Value> = serde_json::from_str(&claims_str)?;

        Ok(claims)
    }
}
