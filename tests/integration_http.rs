use json_sender::files::Files;
use json_sender::http::HTTP;
use json_sender::settings::Settings;

use std::path::Path;

mod common;

#[tokio::test]
async fn http() {
    common::setup();

    let settings = Settings::new(Some("mock/sender.toml".to_owned()));

    let files = Files::new(settings.target.clone(), settings.bindinds.clone());
    let mut http = HTTP::new(settings.clone());
    http.use_auth(settings.auth);

    let file_list = files.get_req_info_list();

    for f in file_list {
        http.handle(f).await;
    }

    let exists = Path::new("mock/files/success/POST_POSTS_20220101.json")
        .try_exists()
        .unwrap();

    assert!(exists);

    common::end();
}
