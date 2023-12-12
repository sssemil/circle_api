use crate::api::CircleClient;

use crate::models::token_get::TokenGetResponse;
use reqwest::Method;
use uuid::Uuid;

impl CircleClient {
    pub async fn get_token_details(
        &self,
        token_id: Uuid,
    ) -> crate::error::Result<TokenGetResponse> {
        let url = format!("{}w3s/tokens/{}", self.base_url, token_id);
        self.send_request(Method::GET, url, None::<()>).await
    }
}
