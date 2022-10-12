use clap::Parser;
use json_sender::files::Files;
use json_sender::http::HTTP;
use json_sender::settings::Settings;

/// A simple program to send HTTP requests from .json files.
#[derive(Parser, Debug)]
#[command(name = "json-sender")]
#[command(author = "Gabriel Fioravante")]
#[command(version="0.1.0", about, long_about = None)]
pub struct Args {
    /// Config file path. Ex: "~/files/sender.toml".
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let settings = Settings::new(args.config);

    let files = Files::new(settings.target.clone(), settings.bindinds.clone());
    let mut http = HTTP::new(settings.clone());
    http.use_auth(settings.auth);

    let file_list = files.get_req_info_list();

    for f in file_list {
        http.handle(f).await;
    }
}
