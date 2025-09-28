mod app;
mod cli;
mod configuration;
use crate::cli::run;
use clap::Parser;
use std::process::exit;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args = crate::cli::Args::parse();
    exit(run(args).await)
}
