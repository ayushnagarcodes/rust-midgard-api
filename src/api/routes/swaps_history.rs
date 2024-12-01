use crate::{
    api::{get_history, ApiParams},
    models::SwapsHistory,
};
use axum::{
    extract::{Query, State},
    response::Json,
};
use reqwest::StatusCode;
use sqlx::PgPool;

pub async fn get_swaps_history(
    state: State<PgPool>,
    params: Query<ApiParams>,
) -> Result<Json<Vec<SwapsHistory>>, (StatusCode, String)> {
    get_history::<SwapsHistory>(state, params, "swaps_history").await
}
