use config::Config;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Basic {
    pub username: String,
    pub password: Option<String>
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
    pub target: String,
    pub bindinds: HashMap<String, String>,
    pub auth: Option<Auth>,
}

impl Settings {
    pub fn new(custom_path: Option<String>) -> Self {
        let path = custom_path.unwrap_or_else(|| "sender".to_owned());

        let settings = Config::builder()
            .add_source(config::File::with_name(&path))
            .build()
            .unwrap()
            .try_deserialize::<Settings>();

        match settings {
            Ok(s) => s,
            Err(e) => panic!("Failed to parse config file: {}", e),
        }
    }
}
