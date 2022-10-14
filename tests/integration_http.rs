use json_sender::files::{Files, Targets};
use json_sender::http::HTTP;
use json_sender::settings::Settings;
use std::sync::Arc;
use std::time::Instant;

use std::path::Path;

mod common;

// TODO: Find a better way to write this test
#[tokio::test]
async fn http() {
    common::setup();

    let settings = Settings::new(Some("mock/sender.toml".to_owned())).unwrap();

    // Process files
    let measure_file = Instant::now();

    let files = Files::new(
        Targets {
            param: None,
            config: settings.target.clone(),
        },
        settings.bindinds.clone(),
    ).unwrap();

    let file_list = files.list().unwrap();
    let files_duration = measure_file.elapsed();

    log::info!("Processed files in: {:?}", files_duration);

    assert_eq!(file_list.len(), 3);

    // Send requests
    let http = Arc::new(HTTP::new(settings));
    let measure_requests = Instant::now();

    for f in file_list {
        let h = Arc::clone(&http);
        tokio::spawn(async move {
            if (h.handle(f).await).is_ok() {}
        })
        .await
        .unwrap();
    }

    let requests_duration = measure_requests.elapsed();
    log::info!("Sent requests in: {:?}", requests_duration);

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
