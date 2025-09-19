mod app;
mod cli;
mod configuration;

use std::process::exit;
use crate::cli::run;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = crate::cli::Args::parse();
    exit(run(args).await)
}
