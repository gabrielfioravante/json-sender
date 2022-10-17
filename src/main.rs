use json_sender::{args::Args, http::HTTP, parser::FileParser, settings::Settings, setup};

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup
    let args = Args::parse();
    let settings = Settings::new(args.config)?;

    let target = setup::select_target(setup::Targets {
        param: args.target,
        config: &settings.target,
    })?;

    // Manage log visibility
    std::env::set_var("RUST_LOG", "INFO");
    if args.silent {
        std::env::set_var("RUST_LOG", "OFF");
    }

    env_logger::init();
    log::info!("Starting JSON Sender");

    // Process files
    let parser = FileParser::new(&target, &settings.bindinds)?;

    let measure_parser = Instant::now();
    let file_list = parser.list_files()?;

    log::info!("Processed files in: {:?}", measure_parser.elapsed());
    log::info!("{} requests to send", file_list.len());

    // Should send requests or not
    if file_list.is_empty() {
        log::info!("Ending JSON Sender...");
    } else {
        setup::create_dirs(&target)?;

        // Send requests
        let http = Arc::new(HTTP::new(settings));
        let measure_requests = Instant::now();

        for f in file_list {
            let h = Arc::clone(&http);
            tokio::spawn(async move { if (h.handle(f).await).is_ok() {} }).await?
        }

        log::info!("Sent requests in: {:?}", measure_requests.elapsed());
    };

    Ok(())
}
