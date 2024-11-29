use db::init_db;
use dotenv::dotenv;
use populate_db::populate_db;

mod db;
mod midgard_api;
mod models;
mod populate_db;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    utils::init_tracing();

    // Initialize database pool
    let db_pool = match init_db().await {
        Ok(db) => {
            println!("\nConnected to database!\n");
            db
        }
        Err(error) => {
            eprintln!("Failed to initialize database | {error}");
            return;
        }
    };

    // Fetch data from Midgard API and insert into database
    populate_db(&db_pool).await;
}
