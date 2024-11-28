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
            println!("Connected to database!");
            db
        }
        Err(error) => {
            eprintln!("Failed to initialize database: {error}");
            return;
        }
    };

    // Testing handlers
    let rune_pool_history = handlers::fetch_rune_pool_history().await;
    match rune_pool_history {
        Ok(data) => {
            println!("intervals: {:#?}", data);
        }
        Err(error) => {
            eprintln!("Failed to fetch intervals: {error}");
        }
    }
}
