use crate::files::ReqInfo;
use crate::http::{Request, RequestValidation, HTTP, AuthType};

pub struct Put {}

impl Request for Put {
    fn make(&self, http: &HTTP, info: ReqInfo) -> RequestValidation {
        let json = info.read_file();

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
