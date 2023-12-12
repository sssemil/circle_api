use crate::models::token_info::TokenInfo;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenGetResponse {
    pub token: TokenInfo,
}
