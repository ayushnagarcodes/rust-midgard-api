use chrono::{NaiveDate, Timelike, Utc};
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::registry()
        .with(LevelFilter::from_level(Level::TRACE))
        .with(fmt::Layer::default());

    subscriber.init();
}

pub fn truncate_to_hour() -> chrono::DateTime<Utc> {
    let now = Utc::now();

    now.date_naive()
        .and_hms_opt(now.hour(), 0, 0)
        .expect("Failed to truncate to hour")
        .and_utc()
}

pub fn parse_date_to_utc(date: &str) -> Option<chrono::DateTime<Utc>> {
    Some(
        NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .expect("Failed to parse start date")
            .and_hms_opt(0, 0, 0)
            .expect("Failed to parse start time")
            .and_utc(),
    )
}
