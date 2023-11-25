use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceQueryParams {
    include_all: Option<bool>,
    name: Option<String>,
    token_address: Option<String>,
    standard: Option<String>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    page_before: Option<String>,
    page_after: Option<String>,
    page_size: Option<u64>,
}

impl WalletBalanceQueryParams {
    pub fn include_all(mut self, value: bool) -> Self {
        self.include_all = Some(value);
        self
    }

    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn token_address<S: Into<String>>(mut self, value: S) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn standard<S: Into<String>>(mut self, value: S) -> Self {
        self.standard = Some(value.into());
        self
    }

    pub fn from(mut self, value: DateTime<Utc>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<Utc>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn page_before<S: Into<String>>(mut self, value: S) -> Self {
        self.page_before = Some(value.into());
        self
    }

    pub fn page_after<S: Into<String>>(mut self, value: S) -> Self {
        self.page_after = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: u64) -> Self {
        self.page_size = Some(value);
        self
    }
}

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
