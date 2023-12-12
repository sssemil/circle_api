use crate::models::token_info::TokenInfo;
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
pub struct WalletBalanceResponse {
    pub token_balances: Vec<TokenBalance>,
}
