use crate::models::auth::Auth;
use crate::models::custody_type::CustodyType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletSetObjectResponse {
    pub id: Uuid,
    pub custody_type: CustodyType,
    pub name: String,
    pub update_date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletSetRequest {
    #[serde(flatten)]
    pub auth: Auth,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletSetResponse {
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

impl WalletSetsQueryParams {
    pub fn new() -> Self {
        WalletSetsQueryParams {
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
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletSet {
    pub id: Uuid,
    pub create_date: DateTime<Utc>,
    pub custody_type: CustodyType,
    pub name: Option<String>,
    pub update_date: DateTime<Utc>,
    pub user_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletSetsResponse {
    pub wallet_sets: Vec<WalletSet>,
}
