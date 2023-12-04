use crate::error::CircleError;
use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::wallet_create::WalletDetail;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletListQueryParams {
    address: Option<String>,
    blockchain: Option<String>,
    wallet_set_id: Option<Uuid>,
    ref_id: Option<String>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    page_before: Option<Uuid>,
    page_after: Option<Uuid>,
    page_size: Option<u8>,
}

impl WalletListQueryParams {
    pub fn address(mut self, value: String) -> Self {
        self.address = Some(value);
        self
    }

    pub fn blockchain(mut self, value: String) -> Self {
        self.blockchain = Some(value);
        self
    }

    pub fn wallet_set_id(mut self, value: Uuid) -> Self {
        self.wallet_set_id = Some(value);
        self
    }

    pub fn ref_id(mut self, value: String) -> Self {
        self.ref_id = Some(value);
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

    pub fn page_before(mut self, value: Uuid) -> Self {
        self.page_before = Some(value);
        self
    }

    pub fn page_after(mut self, value: Uuid) -> Result<Self> {
        if self.page_before.is_some() {
            Err(CircleError::ValueError)?;
        }
        self.page_after = Some(value);
        Ok(self)
    }

    pub fn page_size(mut self, value: u8) -> Result<Self> {
        if self.page_after.is_some() {
            Err(CircleError::ValueError)?
        }
        self.page_size = Some(value);
        Ok(self)
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletListResponse {
    pub wallets: Vec<WalletDetail>
}