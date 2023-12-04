use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::models::wallet_objects::WalletObjectsQueryParams;

pub type WalletNftsQueryParams = WalletObjectsQueryParams;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletNftsResponse {
    pub nfts: Vec<NftBalance>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NftBalance {
    pub amount: u64,
    pub metadata: Option<String>,
    pub nft_token_id: String,
    pub token: NftToken,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NftToken {
    pub update_date: DateTime<Utc>,
}
