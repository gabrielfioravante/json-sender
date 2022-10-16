use json_sender::{
    args::Args,
    http::HTTP,
    parser::{FileParser, Targets},
    settings::Settings,
};

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let settings = Settings::new(args.config)?;

    // Manage log visibility
    std::env::set_var("RUST_LOG", "INFO");
    if args.silent {
        std::env::set_var("RUST_LOG", "OFF");
    }

    env_logger::init();
    log::info!("Starting JSON Sender");

    // Process files
    let parser = FileParser::new(
        Targets {
            param: args.target,
            config: settings.target.clone(),
        },
        settings.bindinds.clone(),
    )?;

    let measure_parser = Instant::now();
    let file_list = parser.list_files()?;
    let parser_duration = measure_parser.elapsed();

    log::info!("Processed files in: {:?}", parser_duration);
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
