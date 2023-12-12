use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub id: Uuid,
    pub name: Option<String>,
    pub standard: Option<String>,
    pub blockchain: String,
    pub decimals: Option<i32>,
    pub is_native: bool,
    pub symbol: Option<String>,
    pub token_address: Option<String>,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
}
