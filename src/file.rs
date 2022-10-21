use crate::http::{AuthType, Methods, HTTP};
use anyhow::Result;
use reqwest::RequestBuilder;
use reqwest::Response;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct RequestData {
    pub method: Methods,
    pub endpoint: String,
    pub id: Option<String>,
}

#[derive(Debug)]
pub struct FileData {
    pub path: String,
    pub name: String,
}

#[derive(Debug)]
pub struct FileToSend {
    pub request_data: RequestData,
    pub data: FileData,
    pub write_response: bool,
}

impl FileToSend {
    pub async fn execute(&self, http: &HTTP) -> Result<()> {
        let request_builder = self.make_builder(http).await;
        self.send(request_builder).await?;

        Ok(())
    }

    async fn send(&self, request: Result<RequestBuilder>) -> Result<()> {
        if let Ok(req) = request {
            let res = req.send().await;
            self.manage_result(res).await?;
        } else {
            self.manage_error().await?
        }

        Ok(())
    }

    async fn manage_result(&self, result: reqwest::Result<Response>) -> Result<()> {
        match result {
            Ok(res) => {
                if res.status().is_success() {
                    self.manage_success().await?
                } else {
                    self.manage_error().await?
                }

                if self.write_response {
                    self.write_response_to_file(res).await?
                };
            }
            Err(_) => self.manage_error().await?,
        }

        Ok(())
    }

    async fn write_response_to_file(&self, response: Response) -> Result<()> {
        let mut file_path = self.data.path.replace(&self.data.name, "");
        file_path.push_str("response/");
        file_path.push_str(&(response.status().as_u16().to_string() + "_"));
        file_path.push_str(&self.data.name);

        let text = response.bytes().await?;

        let mut new_file = File::create(file_path).await?;
        new_file.write_all(&text).await?;

        Ok(())
    }

    async fn manage_error(&self) -> Result<()> {
        self.move_to_folder("error/").await?;
        log::error!("`{}` went wrong!", self.data.name);

        Ok(())
    }

    async fn manage_success(&self) -> Result<()> {
        self.move_to_folder("success/").await?;
        log::info!("`{}` sent!", self.data.name);

        Ok(())
    }

    async fn move_to_folder(&self, folder: &str) -> Result<()> {
        let mut new_path = self.data.path.replace(&self.data.name, "");

        new_path.push_str(folder);
        new_path.push_str(&self.data.name);

        tokio::fs::rename(&self.data.path, new_path).await?;

        Ok(())
    }

    async fn make_builder(&self, http: &HTTP) -> Result<RequestBuilder> {
        let client = &http.client;
        let mut builder: RequestBuilder;

        match self.get_method() {
            Methods::GET => {
                builder = client
                    .get(http.generate_url(&self.request_data.endpoint, &self.request_data.id));
            }
            Methods::POST => {
                let json = self.read_file().await?;
                builder = client
                    .post(http.generate_url(&self.request_data.endpoint, &self.request_data.id))
                    .body(json);
            }
            Methods::PUT => {
                let json = self.read_file().await?;
                builder = client
                    .put(http.generate_url(&self.request_data.endpoint, &self.request_data.id))
                    .body(json);
            }
            Methods::DELETE => {
                builder = client
                    .delete(http.generate_url(&self.request_data.endpoint, &self.request_data.id));
            }
        }

        if let AuthType::None = http.auth {
            builder = self.set_auth(builder, http);
        };

        Ok(builder)
    }

    async fn read_file(&self) -> Result<String> {
        let file_content = tokio::fs::read_to_string(&self.data.path).await?;
        Ok(file_content)
    }

    pub fn get_method(&self) -> &Methods {
        &self.request_data.method
    }

    fn set_auth(&self, builder: RequestBuilder, http: &HTTP) -> RequestBuilder {
        match &http.auth {
            AuthType::Bearer(token) => builder.bearer_auth(token),
            AuthType::Basic { user, pass } => builder.basic_auth(user, pass.as_ref()),
            AuthType::None => builder,
        }
    }
}
