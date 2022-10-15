use crate::files::ReqInfo;
use crate::http::{AuthType, Request, RequestValidation, HTTP};
use async_trait::async_trait;

pub struct Put;

#[async_trait]
impl Request for Put {
    async fn make(&self, http: &HTTP, info: ReqInfo) -> RequestValidation {
        let json = info.read_file().await;

        match json {
            Ok(j) => {
                let mut builder = http
                    .client
                    .put(http.generate_url_with_id(&info.metadata.endpoint, &info.metadata.id))
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
