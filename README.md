# JSON Sender
A simple program to send HTTP requests from .json files.

## Install 
At this point, the only way to use this program is building it and getting the executable at `target/release`.

```sh
git clone https://github.com/gabrielfioravante/json-sender
cd json-sender
cargo build --release
cd target/release

```

## Usage
By default, json-sender looks for a configuration file in the current directory, but optionally you can pass a path to other config file via command line argument.

```sh
Usage: json-sender [OPTIONS]

Options:
  -c, --config <CONFIG>  Config file path. Ex: "~/files/sender.toml"
  -h, --help             Print help information
  -V, --version          Print version information
```

## Configuration
TOML example:
```toml
base_url = "https://jsonplaceholder.typicode.com"
target = "example/files_to_send"

# You can setup authentication
[auth]

[auth.bearer]
token="Some token"

# Will override Bearer
[auth.basic] 
username="Gabriel"
password="123"

# You MUST bind some key to an API endpoint.
[bindinds]
USERS = "/users"
POSTS = "/posts"
RANDOM = "/random_endpoints"
```

## Files
Every file name must be composed of `<METHOD>_<KEY>_<OPTIONAL ID>_<DATETIME || SOMETHING UNIQUE>.json`.

Examples:
```
POST_POSTS_1644806288.json
GET_USERS_1645921047.json
PUT_POSTS_13_1652633341.json
DELETE_POSTS_22_1659582053.json
```

## Folder structure
Files in which the requests succeeded will be moved to the `success` folder. The ones that failed, to the `failed` folder. These folders are created automatically.

```
files_to_send
├── DELETE_POSTS_22_1659582053.json
├── POST_POSTS_1644806288.json
├── PUT_POSTS_13_1652633341.json
├── error
│   └── GET_USERS_1653788628.json
└── success
    ├── DELETE_POSTS_17_1659582053.json
    └── GET_USERS_1645921047.json
```

## Features to implement
- [ ] Generate files with requests responses
- [ ] Add target parameter to CLI
