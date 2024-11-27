use dotenv::dotenv;

mod db;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    utils::init_tracing();

    // Initialize database pool
    let db_pool = db::init_db().await.expect("Failed to initialize database");
}
