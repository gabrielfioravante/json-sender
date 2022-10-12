use crate::files::ReqInfo;
use crate::http::{Request, RequestValidation, HTTP, AuthType};

pub struct Get {}

impl Request for Get {
    fn make(&self, http: &HTTP, info: ReqInfo) -> RequestValidation {
        let mut builder = http
            .client
            .get(http.generate_url_with_id(&info.metadata.endpoint, &info.metadata.id));


        if let AuthType::None = http.auth {
            builder = self.set_auth(builder, http)
        }

        RequestValidation::Valid(Box::new(builder), info)
    }
}
