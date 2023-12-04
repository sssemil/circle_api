use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::models::wallet_objects::WalletObjectsQueryParams;

pub type WalletBalanceQueryParams = WalletObjectsQueryParams;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub amount: String,
    pub token: TokenInfo,
    pub update_date: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub id: String,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResponse {
    pub token_balances: Vec<TokenBalance>,
}
