use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::pagination::Pagination;
use crate::models::time_range::TimeRange;
use crate::models::wallet_detail::WalletDetail;
use crate::models::web3_address::Web3Address;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletListQueryParams {
    address: Option<Web3Address>,
    blockchain: Option<String>,
    wallet_set_id: Option<Uuid>,
    ref_id: Option<String>,
    #[serde(flatten)]
    time_range: TimeRange,
    #[serde(flatten)]
    pagination: Pagination,
}

impl WalletListQueryParams {
    pub fn address(mut self, value: Web3Address) -> Self {
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
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletListResponse {
    pub wallets: Vec<WalletDetail>,
}
