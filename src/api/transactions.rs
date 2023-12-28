use reqwest::Method;

use uuid::Uuid;

use crate::api::{encrypt_entity_secret, CircleClient};

use crate::error::Result;
use crate::models::auth::Auth;
use crate::models::transaction::TxType;
use crate::models::transaction_accelerate::{
    TransactionAccelerateRequest, TransactionAccelerateResponse,
};
use crate::models::transaction_get::TransactionGetResponse;
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

    pub async fn get_transaction(
        &self,
        transaction_id: Uuid,
        tx_type: Option<TxType>,
    ) -> Result<TransactionGetResponse> {
        let tx_type_param = match tx_type {
            Some(tx_type) => format!("/{}", serde_qs::to_string(&tx_type)?),
            None => String::new(),
        };
        let url = format!(
            "{}w3s/transactions/{}{}",
            self.base_url, transaction_id, tx_type_param
        );
        self.send_request(Method::GET, url, None::<()>).await
    }

    // TODO: estimate fee for a transaction
    // TODO: estimate fee for a contract execution transaction
    // TODO: validate an address
}
