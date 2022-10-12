use crate::{files::Files, http::HTTP, settings::Settings};
use clap::Parser;
use std::sync::Arc;

pub mod files;
pub mod http;
pub mod settings;

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

pub async fn init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting JSON Sender");

    let args = Args::parse();
    let settings = Settings::new(args.config);

    let files = Files::new(settings.target.clone(), settings.bindinds.clone());
    let http = Arc::new(HTTP::new(settings));

    let file_list = files.get_req_info_list();

    log::info!("{} requests to send", file_list.len());

    for f in file_list {
        let h = Arc::clone(&http);
        tokio::spawn(async move {
            h.handle(f).await;
        })
        .await
        .unwrap();
    }
}
