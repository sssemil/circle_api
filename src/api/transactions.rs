use reqwest::Method;

use crate::api::CircleClient;

use crate::error::Result;
use crate::models::transaction_accelerate::{
    TransactionAccelerateRequest, TransactionAccelerateResponse,
};
use crate::models::transaction_transfer_create::{
    TransactionTransferCreateRequest, TransactionTransferCreateResponse,
};

impl CircleClient {
    pub async fn create_transfer_transaction(
        &self,
        request: TransactionTransferCreateRequest,
    ) -> Result<TransactionTransferCreateResponse> {
        let url = format!("{}w3s/developer/transactions/transfer", self.base_url);
        let r = self.send_request(Method::POST, url, Some(request)).await?;
        Ok(r)
    }

    pub async fn accelerate_transaction(
        &self,
        transaction_id: String,
        request: TransactionAccelerateRequest,
    ) -> Result<TransactionAccelerateResponse> {
        let url = format!(
            "{}w3s/developer/transactions/{}/accelerate",
            self.base_url, transaction_id
        );
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
