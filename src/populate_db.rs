use crate::db::insertions;
use crate::midgard_api::{self, handlers};
use crate::utils::truncate_to_hour;
use chrono::Duration;
use sqlx::PgPool;

pub fn midgard_params(iteration: i64) -> midgard_api::Params {
    midgard_api::Params {
        interval: "hour".to_string(),
        from: truncate_to_hour() - Duration::days(16 * iteration),
        to: truncate_to_hour() - Duration::days(16 * (iteration - 1)),
    }
}

pub async fn populate_db(db_pool: &PgPool) {
    println!("Populating database...");
    let total_iteration: i64 = 5;
    let mut current_iteration: i64 = 1;

    while current_iteration <= total_iteration {
        let params = midgard_params(total_iteration - current_iteration + 1);
        println!(
            "\n\n------------Iteration: {} | From: {} | To: {}------------\n",
            current_iteration, &params.from, &params.to
        );

        let depth_price_history = match handlers::fetch_depth_price_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch depth price history | {error}");
                return;
            }
        };
        match insertions::insert_depth_price_history(db_pool, &depth_price_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert depth price history: {error}");
                return;
            }
        };

        let earnings_history = match handlers::fetch_earnings_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch earnings history | {error}");
                return;
            }
        };
        match insertions::insert_earnings_history(db_pool, &earnings_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert earnings history: {error}");
                return;
            }
        };

        let rune_pool_history = match handlers::fetch_rune_pool_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch rune pool history | {error}");
                return;
            }
        };
        match insertions::insert_rune_pool_history(db_pool, &rune_pool_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert rune pool history: {error}");
                return;
            }
        };

        let swaps_history = match handlers::fetch_swaps_history(params.clone()).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to fetch swaps history | {error}");
                return;
            }
        };
        match insertions::insert_swaps_history(db_pool, &swaps_history).await {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to insert swaps history: {error}");
            }
        };

        current_iteration += 1;
    }
}
