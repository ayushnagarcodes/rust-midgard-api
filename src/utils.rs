use chrono::{Timelike, Utc};

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
}

pub fn truncate_to_hour() -> chrono::DateTime<Utc> {
    let now = Utc::now();

    now.date_naive()
        .and_hms_opt(now.hour(), 0, 0)
        .unwrap()
        .and_utc()
}
