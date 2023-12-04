use serde::Deserialize;

use crate::models::wallet_detail::WalletDetail;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletGetResponse {
    pub wallet: WalletDetail,
}
