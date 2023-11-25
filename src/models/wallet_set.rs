use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletSetObjectResponse {
    pub id: Uuid,
    pub custody_type: String,
    pub name: String,
    pub update_date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletSetRequest {
    pub idempotency_key: Uuid,
    pub entity_secret_cipher_text: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletSetResponse {
    pub wallet_set: WalletSetObjectResponse,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletSetRequest {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWalletSetResponse {
    pub wallet_set: WalletSetObjectResponse,
}

#[derive(Serialize, Debug, Default)]
pub struct WalletSetsQueryParams {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub page_before: Option<String>,
    pub page_after: Option<String>,
    pub page_size: Option<u64>,
}

pub type WalletSetsQueryParamsBuilder = WalletSetsQueryParams;

impl WalletSetsQueryParamsBuilder {
    pub fn new() -> Self {
        WalletSetsQueryParamsBuilder {
            from: None,
            to: None,
            page_before: None,
            page_after: None,
            page_size: None,
        }
    }

    pub fn from(mut self, from: DateTime<Utc>) -> Self {
        self.from = Some(from);
        self
    }

    pub fn to(mut self, to: DateTime<Utc>) -> Self {
        self.to = Some(to);
        self
    }

    pub fn page_before<S: Into<String>>(mut self, page_before: S) -> Self {
        self.page_before = Some(page_before.into());
        self
    }

    pub fn page_after<S: Into<String>>(mut self, page_after: S) -> Self {
        self.page_after = Some(page_after.into());
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn build(self) -> WalletSetsQueryParams {
        WalletSetsQueryParams {
            from: self.from,
            to: self.to,
            page_before: self.page_before,
            page_after: self.page_after,
            page_size: self.page_size,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletSet {
    pub id: String,
    pub create_date: DateTime<Utc>,
    pub custody_type: String,
    pub name: Option<String>,
    pub update_date: DateTime<Utc>,
    pub user_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletSetsResponse {
    pub wallet_sets: Vec<WalletSet>,
}
