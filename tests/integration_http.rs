use json_sender::{http::HTTP, parser::FileParser, settings::Settings, setup};
use std::sync::Arc;
use std::time::Instant;

use std::path::Path;

mod common;

// TODO: Find a better way to write this test
#[tokio::test]
async fn http() {
    // Setup
    common::setup();
    let settings = Settings::new(Some("mock/sender.toml".to_owned())).unwrap();

    let target = setup::select_target(&None, &settings.target).unwrap();

    // Process files
    let parser = FileParser::new(target, &settings.bindinds, settings.write_response).unwrap();

    let measure_parser = Instant::now();
    let file_list = parser.list_files().unwrap();
    let parser_duration = measure_parser.elapsed();

    log::info!("Processed files in: {:?}", parser_duration);
    log::info!("{} requests to send", file_list.len());

    assert_eq!(file_list.len(), 3);

    // Send requests
    let http = Arc::new(HTTP::new(settings));
    let measure_requests = Instant::now();

    for f in file_list {
        let h = Arc::clone(&http);
        tokio::spawn(async move { if (h.handle(f).await).is_ok() {} })
            .await
            .unwrap();
    }

    let requests_duration = measure_requests.elapsed();
    log::info!("Sent requests in: {:?}", requests_duration);

    test_post();
    test_get();
    test_put();

    test_post_response();
    test_get_response();
    test_put_response();

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

fn test_post_response() {
    let path = "mock/files/success/POST_POSTS_1644806288.json";
    let file = Path::new(path).try_exists().unwrap();

    assert!(file);
    assert!(!std::fs::read(path).unwrap().is_empty())
}

fn test_get_response() {
    let path = "mock/files/response/200_PUT_USERS_1_1644806288.json";
    let file = Path::new(path).try_exists().unwrap();

    assert!(file);
    assert!(!std::fs::read(path).unwrap().is_empty())
}

fn test_put_response() {
    let path = "mock/files/success/PUT_USERS_1_1644806288.json";
    let file = Path::new(path).try_exists().unwrap();

    assert!(file);
    assert!(!std::fs::read(path).unwrap().is_empty())
}
