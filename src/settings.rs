use anyhow::{Context, Result};
use config::Config;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Basic {
    pub username: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Bearer {
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub bearer: Option<Bearer>,
    pub basic: Option<Basic>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub base_url: String,
    pub target: Option<String>,
    pub bindinds: HashMap<String, String>,
    pub auth: Option<Auth>,
}

impl Settings {
    pub fn new(custom_path: Option<String>) -> Result<Self> {
        let path = custom_path.unwrap_or_else(|| "sender.toml".to_owned());

        let settings = Config::builder()
            .add_source(config::File::with_name(&path))
            .build()?
            .try_deserialize::<Settings>()
            .context("Unable to parse configuration file.")?;

        Ok(settings)
    }
}
