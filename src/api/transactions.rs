use reqwest::Method;
use uuid::Uuid;

use crate::api::{encrypt_entity_secret, CircleClient};

use crate::error::Result;
use crate::models::auth::Auth;
use crate::models::transaction_accelerate::{
    TransactionAccelerateRequest, TransactionAccelerateResponse,
};
use crate::models::transaction_transfer_create::{
    TransactionTransferCreateRequestBuilder, TransactionTransferCreateResponse,
};

impl CircleClient {
    pub async fn create_transfer_transaction(
        &self,
        idempotency_key: Uuid,
        request: TransactionTransferCreateRequestBuilder,
    ) -> Result<TransactionTransferCreateResponse> {
        let url = format!("{}w3s/developer/transactions/transfer", self.base_url);
        let request = request.build(Auth::new(
            idempotency_key,
            encrypt_entity_secret(&self.public_key, &self.circle_entity_secret)?,
        ));
        let response = self.send_request(Method::POST, url, Some(request)).await?;
        Ok(response)
    }

    pub async fn accelerate_transaction(
        &self,
        transaction_id: String,
        idempotency_key: Uuid,
    ) -> Result<TransactionAccelerateResponse> {
        let url = format!(
            "{}w3s/developer/transactions/{}/accelerate",
            self.base_url, transaction_id
        );
        let request = TransactionAccelerateRequest {
            auth: Auth::new(
                idempotency_key,
                encrypt_entity_secret(&self.public_key, &self.circle_entity_secret)?,
            ),
        };
        let response = self.send_request(Method::POST, url, Some(request)).await?;
        Ok(response)
    }

    // TODO: cancel a transaction
    // TODO: create a contract execution transaction
    // TODO: list transactions
    // TODO: get a transaction
    // TODO: estimate fee for a transaction
    // TODO: estimate fee for a contract execution transaction
    // TODO: validate an address
}
