use crate::{
    api::{get_history, ApiParams},
    models::RunePoolHistory,
};
use axum::{
    extract::{Query, State},
    response::Json,
};
use reqwest::StatusCode;
use sqlx::PgPool;

pub async fn get_rune_pool_history(
    state: State<PgPool>,
    params: Query<ApiParams>,
) -> Result<Json<Vec<RunePoolHistory>>, (StatusCode, String)> {
    get_history::<RunePoolHistory>(state, params, "rune_pool_history").await
}