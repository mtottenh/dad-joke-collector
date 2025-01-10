mod config;
mod db;
mod models;

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use std::time::Duration;
use tokio::time;
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::{self, StreamExt};
use tracing::{info, error};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to config file. If not specified, will use XDG config directory
    #[arg(short, long)]
    config: Option<String>,
}

use crate::config::Config;
use crate::db::Database;
use crate::models::DadJoke;

async fn fetch_dad_joke(client: &Client) -> Result<DadJoke> {
    let response = client
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .header("User-Agent", "Enthusiastic Dad Joke Collector")
        .send()
        .await?
        .json::<DadJoke>()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting Dad Jokes Collector");

    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let config = Config::load(args.config.as_deref()).context("Failed to load configuration")?;
    info!("Configuration loaded");

    // Initialize database
    let db = Database::new(&config.database_path).context("Failed to initialize database")?;
    info!("Database initialized at {}", config.database_path);

    // Calculate interval based on rate limit
    let interval_seconds = 60 / config.rate_limit_per_minute;
    info!("Rate limit set to {} requests per minute", config.rate_limit_per_minute);

    // Set up signal handler for graceful shutdown
    let mut sigterm = signal(SignalKind::terminate())?;
    let client = Client::new();
    let mut interval = tokio_stream::wrappers::IntervalStream::new(
        time::interval(Duration::from_secs(interval_seconds))
    );

    loop {
        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM, shutting down gracefully...");
                break;
            }
            Some(_) = interval.next() => {
                match fetch_dad_joke(&client).await {
                    Ok(joke) => {
                        match db.insert_joke(&joke) {
                            Ok(true) => info!("New joke stored: {}", joke.joke),
                            Ok(false) => info!("Joke already exists: {}", joke.id),
                            Err(e) => error!("Failed to store joke: {}", e),
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch joke: {}", e);
                        time::sleep(Duration::from_secs(30)).await;
                    }
                }
            }
            else => break,
        }
    }

    info!("Shutdown complete");
    Ok(())
}
