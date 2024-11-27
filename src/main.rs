use dotenv::dotenv;

mod db;
mod models;
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
}
