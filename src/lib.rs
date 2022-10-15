use crate::files::Targets;
use crate::{files::Files, http::HTTP, settings::Settings};
use anyhow::Result;
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
    /// Target folder path. Ex: "example/files/".
    #[arg(short, long)]
    target: Option<String>,

    /// Config file path. Ex: "example/sender.toml".
    #[arg(short, long)]
    config: Option<String>,

    /// Turn off logging
    #[arg(short, long)]
    silent: bool,
}

pub async fn init() -> Result<()> {
    let args = Args::parse();
    let settings = Settings::new(args.config)?;

    std::env::set_var("RUST_LOG", "INFO");
    if args.silent {
        std::env::set_var("RUST_LOG", "OFF");
    }

    env_logger::init();
    log::info!("Starting JSON Sender");

    // Process files
    let files = Files::new(
        Targets {
            param: args.target,
            config: settings.target.clone(),
        },
        settings.bindinds.clone(),
    )?;

    let measure_file = Instant::now();
    let file_list = files.list()?;
    let files_duration = measure_file.elapsed();

    log::info!("Processed files in: {:?}", files_duration);
    log::info!("{} requests to send", file_list.len());

    // Send requests
    let http = Arc::new(HTTP::new(settings));
    let measure_requests = Instant::now();

    for f in file_list {
        let h = Arc::clone(&http);
        tokio::spawn(async move { if (h.handle(f).await).is_ok() {} }).await?
    }

    let requests_duration = measure_requests.elapsed();
    log::info!("Sent requests in: {:?}", requests_duration);

    Ok(())
}
