use json_sender::settings::Settings;

mod common;

#[test]
fn settings() {
    common::setup();

    let settings = Settings::new(Some("mock/sender.toml".to_owned()));
    assert_eq!(settings.base_url, "https://jsonplaceholder.typicode.com".to_owned());
    assert_eq!(settings.target, "mock/files".to_owned());
    assert_eq!(settings.bindinds.get("POSTS").unwrap(), &"/posts".to_owned());

    common::cleanup();
}
