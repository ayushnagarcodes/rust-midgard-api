use crate::midgard_api;
use chrono::{Duration, Utc};

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
}

pub fn midgard_params() -> midgard_api::Params {
    midgard_api::Params {
        interval: "hour".to_string(),
        from: Utc::now() - Duration::days(30),
        to: Utc::now(),
    }
}
