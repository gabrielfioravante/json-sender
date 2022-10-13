use std::fs::File;
use std::fs;
use std::io::{Error, Write};

//TODO: Find a better way to setup things before tests
pub fn setup() {
    fs::create_dir_all("mock/files").unwrap();
    fs::create_dir_all("mock/files/success").unwrap();
    fs::create_dir_all("mock/files/error").unwrap();
    create_config_file().unwrap();
    create_post_file().unwrap();
    create_get_file().unwrap();
    create_put_file().unwrap();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}

//TODO: Find a better way to cleanup things after tests
pub fn cleanup() {
    fs::remove_dir_all("mock").unwrap();
}

fn create_config_file() -> Result<(), Error> {
    let mut file = File::create("mock/sender.toml")?;
    write!(
        file,
        r#"
base_url = "https://jsonplaceholder.typicode.com"
target = "mock/files"

[bindinds]
POSTS = "/posts"
USERS = "/users"
"#
    )?;

    Ok(())
}

fn create_post_file() -> Result<(), Error> {
    let mut file = File::create("mock/files/POST_POSTS_1644806288.json")?;
    write!(
        file,
        r#"{{"title": "foo", "body": "bar", "userId": 1 }}"#
    )?;

    Ok(())
}

fn create_get_file() -> Result<(), Error> {
    let mut file = File::create("mock/files/GET_USERS_1644806288.json")?;
    write!(
        file,
        r#""#
    )?;

    Ok(())
}


fn create_put_file() -> Result<(), Error> {
    let mut file = File::create("mock/files/PUT_USERS_1_1644806288.json")?;
    write!(
        file,
        r#"{{"name": "gabriel"}}"#
    )?;

    Ok(())
}
