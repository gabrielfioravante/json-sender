use clap::Parser;

/// A simple program to send HTTP requests from .json files.
#[derive(Parser, Debug)]
#[command(name = "json-sender")]
#[command(author = "Gabriel Fioravante")]
#[command(version="0.1.0", about, long_about = None)]
pub struct Args {
    /// Target folder path. Ex: "example/files/".
    #[arg(short, long)]
    pub target: Option<String>,

    /// Config file path. Ex: "example/sender.toml".
    #[arg(short, long)]
    pub config: Option<String>,

    /// Turn off logging
    #[arg(short, long)]
    pub silent: bool,
}

