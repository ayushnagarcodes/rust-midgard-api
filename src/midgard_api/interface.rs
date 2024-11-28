use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub struct Params {
    pub interval: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

pub struct Interface {
    pub resource: String,
    pub params: Params,
}

impl Interface {
    pub fn new(resource: String, params: Params) -> Self {
        Self { resource, params }
    }

    pub async fn fetch_data<T>(&self) -> Result<Vec<T>, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        // Build URL
        let base_url = "https://midgard.ninerealms.com/v2/history";

        let mut url = reqwest::Url::parse(&format!("{}/{}", base_url, self.resource))
            .expect("Invalid base URL or resource");

        url.query_pairs_mut()
            .append_pair("interval", &self.params.interval)
            .append_pair("from", &self.params.from.timestamp().to_string())
            .append_pair("to", &self.params.to.timestamp().to_string());

        let full_url = url.to_string();
        println!("Full URL: {}", full_url);

        // Fetch data
        let resp: Value = reqwest::get(url).await?.json().await?;

        // Extract intervals
        let intervals: Vec<T> = serde_json::from_value(resp["intervals"].clone())
            .expect("Expected 'intervals' to be present and of the correct type");

        Ok(intervals)
    }
}
