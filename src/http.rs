use crate::file::FileToSend;
use crate::settings::{Auth, Settings};
use anyhow::Result;

#[derive(Debug)]
pub enum Methods {
    GET,
    POST,
    PUT,
    DELETE,
}

pub enum AuthType {
    Bearer(String),
    Basic { user: String, pass: Option<String> },
    None,
}

pub struct HTTP {
    pub client: reqwest::Client,
    pub base_url: String,
    pub auth: AuthType,
}

impl HTTP {
    pub fn new(settings: Settings) -> Self {
        HTTP {
            client: reqwest::Client::new(),
            base_url: settings.base_url,
            auth: HTTP::manage_auth(settings.auth),
        }
    }

    pub async fn handle(&self, file: FileToSend) -> Result<()> {
        file.execute(self).await?;
        Ok(())
    }

    pub fn generate_url(&self, endpoint: &String) -> String {
        format!("{}{}", self.base_url, endpoint)
    }

    pub fn generate_url_with_id(&self, endpoint: &String, id: &String) -> String {
        format!("{}{}/{}", self.base_url, endpoint, id)
    }

    fn manage_auth(auth_info: Option<Auth>) -> AuthType {
        if let Some(auth) = auth_info {
            if let Some(bearer) = auth.bearer {
                return AuthType::Bearer(bearer.token);
            };

            if let Some(basic) = auth.basic {
                AuthType::Basic {
                    user: basic.username,
                    pass: basic.password,
                }
            } else {
                AuthType::None
            }
        } else {
            AuthType::None
        }
    }
}
