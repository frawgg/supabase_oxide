use std::collections::HashMap;

use postgrest::Postgrest;

#[derive(Debug, Clone)]
pub struct Client {
    url: String,
    api_key: String,
    db_options: ClientDbOptions,
    auth_options: ClientAuthOptions,
    global_options: ClientGlobalOptions
}

impl Client {
    pub fn new(url: String, api_key: String) -> Self {
        Self {
            url: format!("{url}/rest/v1"),
            api_key: api_key,
            db_options: ClientDbOptions::default(),
            auth_options: ClientAuthOptions::default(),
            global_options: ClientGlobalOptions::default()
        }
    }

    pub fn with_db_options(mut self, options: ClientDbOptions) -> Self {
        self.db_options = options;
        self
    }

    pub fn with_auth_options(mut self, options: ClientAuthOptions) -> Self {
        self.auth_options = options;
        self
    }

    pub fn with_global_options(mut self, options: ClientGlobalOptions) -> Self {
        self.global_options = options;
        self
    }

    pub fn from(&self, table: &str) -> Result<postgrest::Builder, anyhow::Error> {
        let mut client = Postgrest::new(&self.url);
        if let Some(headers) = &self.global_options.headers {
            for (k, v) in headers {
                client = client.insert_header(k.parse::<http::header::HeaderName>()?, v.to_owned());
            }
        }
        let builder = client.from(table).auth(self.api_key.clone());
        Ok(builder)
    }

    pub fn rpc(&self, function: &str, params: &str) -> Result<postgrest::Builder, anyhow::Error> {
        let mut client = Postgrest::new(&self.url);
        if let Some(headers) = &self.global_options.headers {
            for (k, v) in headers {
                client = client.insert_header(k.parse::<http::header::HeaderName>()?, v.to_owned());
            }
        }
        let builder = client.rpc(function, params).auth(self.api_key.clone());
        Ok(builder)
    }
}

#[derive(Debug, Clone)]
pub struct ClientDbOptions {
    pub schema: String
}

impl ClientDbOptions {
    pub fn default() -> Self {
        Self {
            schema: "public".to_owned()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientAuthOptions {
    pub auto_refresh_token: bool,
    pub persist_session: bool,
}

impl ClientAuthOptions {
    pub fn default() -> Self {
        Self {
            auto_refresh_token: true,
            persist_session: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientGlobalOptions {
    pub headers: Option<HashMap<String, String>>,
}

impl ClientGlobalOptions {
    pub fn default() -> Self {
        Self {
            headers: None
        }
    }
}
