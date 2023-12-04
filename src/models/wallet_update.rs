use serde::{Deserialize, Serialize};

use crate::models::wallet_detail::WalletDetail;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletUpdateRequest {
    pub name: String,
    pub ref_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletUpdateResponse {
    pub wallet: WalletDetail,
}
