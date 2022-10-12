use crate::files::ReqInfo;
use crate::http::{Request, RequestValidation, HTTP, AuthType};

pub struct Delete {}

impl Request for Delete {
    fn make(&self, http: &HTTP, info: ReqInfo) -> RequestValidation {
        let mut builder = http
            .client
            .delete(http.generate_url_with_id(&info.metadata.endpoint, &info.metadata.id));

        if let AuthType::None = http.auth {
            builder = self.set_auth(builder, http)
        }

        RequestValidation::Valid(Box::new(builder), info)
    }
}
