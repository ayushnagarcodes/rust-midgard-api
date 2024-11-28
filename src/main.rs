use db::insertions;
use dotenv::dotenv;
use midgard_api::handlers;

mod db;
mod midgard_api;
mod models;
// mod scheduler;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    utils::init_tracing();

    // Initialize database pool
    let db_pool = match db::init_db().await {
        Ok(db) => {
            println!("Connected to database!\n");
            db
        }
        Err(error) => {
            eprintln!("Failed to initialize database: {error}");
            return;
        }
    };

    // Fetch data from Midgard API and insert into database
    let depth_price_history = match handlers::fetch_depth_price_history().await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch depth price history: {error}");
            return;
        }
    };
    match insertions::insert_depth_price_history(&db_pool, &depth_price_history).await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to insert depth price history: {error}");
            return;
        }
    };

    let earnings_history = match handlers::fetch_earnings_history().await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch earnings history: {error}");
            return;
        }
    };
    match insertions::insert_earnings_history(&db_pool, &earnings_history).await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to insert earnings history: {error}");
            return;
        }
    };

    let rune_pool_history = match handlers::fetch_rune_pool_history().await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch rune pool history: {error}");
            return;
        }
    };
    match insertions::insert_rune_pool_history(&db_pool, &rune_pool_history).await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to insert rune pool history: {error}");
            return;
        }
    };

    let swaps_history = match handlers::fetch_swaps_history().await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to fetch swaps history: {error}");
            return;
        }
    };
    match insertions::insert_swaps_history(&db_pool, &swaps_history).await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to insert swaps history: {error}");
            return;
        }
    };
}
