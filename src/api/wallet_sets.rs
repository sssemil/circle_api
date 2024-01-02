use crate::error::Result;
use reqwest::Method;

use crate::api::{encrypt_entity_secret, CircleClient};
use crate::models::auth::Auth;
use uuid::Uuid;

use crate::models::wallet_set::{
    CreateWalletSetRequest, CreateWalletSetResponse, GetWalletSetResponse, UpdateWalletSetRequest,
    UpdateWalletSetResponse, WalletSetsQueryParams, WalletSetsResponse,
};

impl CircleClient {
    pub async fn list_wallet_sets(
        &self,
        query_params: WalletSetsQueryParams,
    ) -> Result<WalletSetsResponse> {
        let url = format!("{}w3s/walletSets", self.base_url);
        self.send_request(Method::GET, url, Some(query_params))
            .await
    }

    pub async fn create_wallet_set(
        &self,
        idempotency_key: Uuid,
        name: String,
    ) -> Result<CreateWalletSetResponse> {
        let url = format!("{}w3s/developer/walletSets", self.base_url);
        let request = CreateWalletSetRequest {
            auth: Auth::new(
                idempotency_key,
                encrypt_entity_secret(&self.public_key, &self.circle_entity_secret)?,
            ),
            name,
        };
        self.send_request(Method::POST, url, Some(request)).await
    }

    pub async fn update_wallet_set(
        &self,
        wallet_set_id: Uuid,
        name: String,
    ) -> Result<UpdateWalletSetResponse> {
        let url = format!(
            "{}w3s/developer/walletSets/{}",
            self.base_url, wallet_set_id
        );
        let request = UpdateWalletSetRequest { name };
        self.send_request(Method::PUT, url, Some(request)).await
    }

    pub async fn get_wallet_set(&self, wallet_set_id: Uuid) -> Result<GetWalletSetResponse> {
        let url = format!("{}w3s/walletSets/{}", self.base_url, wallet_set_id);
        self.send_request(Method::GET, url, None::<()>).await
    }
}
