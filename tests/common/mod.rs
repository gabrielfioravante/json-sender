use std::fs::File;
use std::fs;
use std::io::{Error, Write};

pub fn setup() {
    fs::create_dir_all("mock/files").unwrap();
    fs::create_dir_all("mock/files/success").unwrap();
    fs::create_dir_all("mock/files/error").unwrap();
    create_config_file().unwrap();
    create_post_file().unwrap();
}

pub fn end() {
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

[auth]

[auth.bearer]
token = "some_token"

[auth.basic]
username = "john"
password = "123"
"#
    )?;

    Ok(())
}

fn create_post_file() -> Result<(), Error> {
    let mut file = File::create("mock/files/POST_POSTS_20220101.json")?;
    write!(
        file,
        r#"{{"title": "foo", "body": "bar", "userId": 1 }}"#
    )?;

    Ok(())
}
