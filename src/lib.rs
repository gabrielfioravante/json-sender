use crate::{files::Files, http::HTTP, settings::Settings};
use clap::Parser;
use std::sync::Arc;
use std::time::Instant;

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

    // Process files
    let measure_file = Instant::now();
    let file_list = files.get_req_info_list();
    let files_duration = measure_file.elapsed();

    log::info!("Processed files in: {:?}", files_duration);
    log::info!("{} requests to send", file_list.len());

    // Send requests
    let measure_requests = Instant::now();

    for f in file_list {
        let h = Arc::clone(&http);
        tokio::spawn(async move {
            h.handle(f).await;
        })
        .await
        .unwrap();
    }

    let requests_duration = measure_requests.elapsed();
    log::info!("Sent requests in: {:?}", requests_duration);
}
