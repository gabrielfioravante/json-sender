use json_sender::settings::Settings;

mod common;

#[test]
fn settings() {
    common::setup();

    let settings = Settings::new(Some("mock/sender.toml".to_owned())).unwrap();
    assert_eq!(settings.base_url, "https://jsonplaceholder.typicode.com".to_string());
    assert_eq!(settings.target.unwrap(), "mock/files".to_string());
    assert_eq!(settings.bindinds.get("POSTS").unwrap(), &"/posts".to_string());

    common::cleanup();
}
