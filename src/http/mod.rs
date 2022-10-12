use crate::files::ReqInfo;
use crate::settings::{Auth, Settings};
use async_trait::async_trait;
use reqwest::{RequestBuilder, Response, Result};

use delete::Delete;
use get::Get;
use post::Post;
use put::Put;

pub mod delete;
pub mod get;
pub mod post;
pub mod put;

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

enum RequestValidation {
    Valid(Box<RequestBuilder>, ReqInfo),
    NotValid(ReqInfo),
}

#[async_trait]
trait Request {
    fn make(&self, http: &HTTP, info: ReqInfo) -> RequestValidation;

    fn set_auth(&self, builder: RequestBuilder, http: &HTTP) -> RequestBuilder {
        match &http.auth {
            AuthType::Bearer(token) => builder.bearer_auth(token),

            AuthType::Basic { user, pass } => builder.basic_auth(user, pass.as_ref()),

            AuthType::None => builder,
        }
    }

    async fn send(&self, req: RequestValidation) {
        match req {
            RequestValidation::Valid(req, info) => {
                let res = req.send().await;
                manage_request_result(res, info)
            }

            RequestValidation::NotValid(info) => manage_error(info),
        }
    }

    async fn handle(&self, http: &HTTP, info: ReqInfo) {
        self.send(self.make(http, info)).await;
    }
}

fn manage_request_result(res: Result<Response>, info: ReqInfo) {
    match res {
        Ok(r) => {
            let status = r.status();

            if status != 200 && status != 201 {
                manage_error(info)
            } else {
                manage_success(info)
            }
        }
        Err(_) => manage_error(info),
    }
}

fn manage_error(info: ReqInfo) {
    info.move_to_folder("error/");
    log::error!("`{}` went wrong!", info.file_data.name)
}

fn manage_success(info: ReqInfo) {
    info.move_to_folder("success/");
    log::info!("`{}` sent!", info.file_data.name)
}

pub struct HTTP {
    client: reqwest::Client,
    base_url: String,
    auth: AuthType,
}

impl HTTP {
    pub fn new(settings: Settings) -> Self {
        HTTP {
            client: reqwest::Client::new(),
            base_url: settings.base_url,
            auth: AuthType::None
        }
    }

    pub fn use_auth(&mut self, auth: Option<Auth>) {
        self.auth = self.manage_auth(auth);
    }

    pub async fn handle(&self, info: ReqInfo) {
        match info.metadata.method {
            Methods::GET => Get {}.handle(self, info).await,
            Methods::POST => Post {}.handle(self, info).await,
            Methods::PUT => Put {}.handle(self, info).await,
            Methods::DELETE => Delete {}.handle(self, info).await,
        }
    }

    pub fn generate_url(&self, endpoint: &String) -> String {
        self.base_url.to_owned() + endpoint
    }

    pub fn generate_url_with_id(&self, endpoint: &String, id: &String) -> String {
        self.base_url.to_owned() + endpoint + "/" + id
    }

    fn manage_auth(&self, auth_info: Option<Auth>) -> AuthType {
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
