use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DepthPriceHistory {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub asset_depth: i64,
    pub rune_depth: i64,
    pub asset_price: f64,
    pub asset_price_usd: f64,
    pub liquidity_units: i64,
    pub members_count: i64,
    pub synth_units: i64,
    pub synth_supply: i64,
    pub units: i64,
    pub luvi: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EarningsHistory {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub liquidity_fees: i64,
    pub block_rewards: i64,
    pub earnings: i64,
    pub bonding_earnings: i64,
    pub liquidity_earnings: i64,
    pub avg_node_count: f64,
    pub rune_price_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PoolEarnings {
    pub earnings_history_id: i64,
    pub pool: String,
    pub asset_liquidity_fees: i64,
    pub rune_liquidity_fees: i64,
    pub total_liquidity_fees_rune: i64,
    pub saver_earning: i64,
    pub rewards: i64,
    pub earnings: i64,
}

pub struct RunePoolHistory {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub depth: i64,
    pub count: i64,
    pub units: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SwapsHistory {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub to_asset_count: i64,
    pub to_rune_count: i64,
    pub to_trade_count: i64,
    pub from_trade_count: i64,
    pub synth_mint_count: i64,
    pub synth_redeem_count: i64,
    pub total_count: i64,
    pub to_asset_volume: i64,
    pub to_rune_volume: i64,
    pub to_trade_volume: i64,
    pub from_trade_volume: i64,
    pub synth_mint_volume: i64,
    pub synth_redeem_volume: i64,
    pub total_volume: i64,
    pub to_asset_volume_usd: i64,
    pub to_rune_volume_usd: i64,
    pub to_trade_volume_usd: i64,
    pub from_trade_volume_usd: i64,
    pub synth_mint_volume_usd: i64,
    pub synth_redeem_volume_usd: i64,
    pub total_volume_usd: i64,
    pub to_asset_fees: i64,
    pub to_rune_fees: i64,
    pub to_trade_fees: i64,
    pub from_trade_fees: i64,
    pub synth_mint_fees: i64,
    pub synth_redeem_fees: i64,
    pub total_fees: i64,
    pub to_asset_average_slip: f64,
    pub to_rune_average_slip: f64,
    pub to_trade_average_slip: f64,
    pub from_trade_average_slip: f64,
    pub synth_mint_average_slip: f64,
    pub synth_redeem_average_slip: f64,
    pub average_slip: f64,
    pub rune_price_usd: f64,
}
