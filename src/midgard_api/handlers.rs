use super::interface::Interface;
use crate::models::{DepthPriceHistory, EarningsHistory, RunePoolHistory, SwapsHistory};
use crate::utils::midgard_params;

pub async fn fetch_depth_price_history() -> Result<Vec<DepthPriceHistory>, reqwest::Error> {
    let params = midgard_params();
    let api_interface = Interface::new("depths/BTC.BTC".to_string(), params);
    println!("Fetching depth price history...");
    api_interface.fetch_data().await
}

pub async fn fetch_earnings_history() -> Result<Vec<EarningsHistory>, reqwest::Error> {
    let params = midgard_params();
    let api_interface = Interface::new("earnings".to_string(), params);
    println!("Fetching earnings history...");
    api_interface.fetch_data().await
}

pub async fn fetch_rune_pool_history() -> Result<Vec<RunePoolHistory>, reqwest::Error> {
    let params = midgard_params();
    let api_interface = Interface::new("runepool".to_string(), params);
    println!("Fetching rune pool history...");
    api_interface.fetch_data().await
}

pub async fn fetch_swaps_history() -> Result<Vec<SwapsHistory>, reqwest::Error> {
    let params = midgard_params();
    let api_interface = Interface::new("swaps".to_string(), params);
    println!("Fetching swaps history...");
    api_interface.fetch_data().await
}
