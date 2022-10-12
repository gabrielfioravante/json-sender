use crate::files::ReqInfo;
use crate::http::{Request, RequestValidation, HTTP, AuthType};

pub struct Post {}

impl Request for Post {
    fn make(&self, http: &HTTP, info: ReqInfo) -> RequestValidation {
        let json = info.read_file();

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
