use json_sender::files::Files;
use json_sender::http::HTTP;
use json_sender::settings::Settings;
use std::sync::Arc;

use std::path::Path;

mod common;

// TODO: Find a better way to write this test
#[tokio::test]
async fn http() {
    common::setup();

    let settings = Settings::new(Some("mock/sender.toml".to_owned()));

    let files = Files::new(settings.target.clone(), settings.bindinds.clone());
    let http = Arc::new(HTTP::new(settings));

    let file_list = files.get_req_info_list();

    assert_eq!(file_list.len(), 3);

    for f in file_list {
        let h = Arc::clone(&http);
        tokio::spawn(async move {
            h.handle(f).await;
        })
        .await
        .unwrap();
    }

    test_post();
    test_get();
    test_put();

    common::cleanup();
}

fn test_post() {
    let post_file = Path::new("mock/files/success/POST_POSTS_1644806288.json")
        .try_exists()
        .unwrap();

    assert!(post_file);
}

fn test_get() {
    let get_file = Path::new("mock/files/success/GET_USERS_1644806288.json")
        .try_exists()
        .unwrap();

    assert!(get_file);
}

fn test_put() {
    let put_file = Path::new("mock/files/success/PUT_USERS_1_1644806288.json")
        .try_exists()
        .unwrap();

    assert!(put_file);
}
