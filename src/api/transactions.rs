use reqwest::Method;

use crate::api::CircleClient;

use crate::error::Result;
use crate::models::transaction::{TransactionRequest, TransactionResponse};

impl CircleClient {
    pub async fn create_transfer_transaction(
        &self,
        request: TransactionRequest,
    ) -> Result<TransactionResponse> {
        let url = format!("{}w3s/developer/transactions/transfer", self.base_url);
        let r = self.send_request(Method::POST, url, Some(request)).await?;
        Ok(r)
    }

    // TODO: accelerate a transaction
    // TODO: cancel a transaction
    // TODO: create a contract execution transaction
    // TODO: list transactions
    // TODO: get a transaction
    // TODO: estimate fee for a transaction
    // TODO: estimate fee for a contract execution transaction
    // TODO: validate an address
}
