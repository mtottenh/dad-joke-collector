use anyhow::{Context, Result};
use dad_jokes_collector::config::Config;
use dad_jokes_collector::db::Database;

fn main() -> Result<()> {
    // Load configuration
    let config = Config::load(None).context("Failed to load configuration")?;

    // Initialize database connection
    let db = Database::new(&config.database_path).context("Failed to connect to database")?;

    // Get and print random joke
    match db.get_random_joke()? {
        Some(joke) => println!("{}", joke),
        None => println!("No jokes found in the database"),
    }

    Ok(())
}

