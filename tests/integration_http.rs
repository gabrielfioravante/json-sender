use std::sync::Arc;
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
    let http = Arc::new(HTTP::new(settings));

    let file_list = files.get_req_info_list();

    for f in file_list {
        let h = Arc::clone(&http);
        tokio::spawn(async move {
            h.handle(f).await;
        }).await.unwrap();
    }

    let exists = Path::new("mock/files/success/POST_POSTS_20220101.json")
        .try_exists()
        .unwrap();

    assert!(exists);

    common::end();
}
