use crate::files::ReqInfo;
use crate::http::{Request, RequestValidation, HTTP, AuthType};
use async_trait::async_trait;

pub struct Post {}

#[async_trait]
impl Request for Post {
    async fn make(&self, http: &HTTP, info: ReqInfo) -> RequestValidation {
        let json = info.read_file().await;

        match json {
            Ok(j) => {
                let mut builder = http
                    .client
                    .post(http.generate_url(&info.metadata.endpoint))
                    .body(j);

                if let AuthType::None = http.auth {
                    builder = self.set_auth(builder, http)
                }

                RequestValidation::Valid(Box::new(builder), info)
            }

            Err(_) => RequestValidation::NotValid(info),
        }
    }
}
