use crate::api::{encrypt_entity_secret, CircleClient};
use crate::error::Result;
use crate::models::signing_message::{SignMessageRequestBuilder, SignMessageResponse};
use crate::models::web3_address::Web3Address;
use crate::models::web3_signature::Web3Signature;
use reqwest::Method;
use uuid::Uuid;
use web3::signing::{keccak256, recover};

impl CircleClient {
    pub async fn sign_message(
        &self,
        request: SignMessageRequestBuilder,
    ) -> Result<SignMessageResponse> {
        let url = format!("{}w3s/developer/sign/message", self.base_url);
        let response = self
            .send_request(
                Method::POST,
                url,
                Some(
                    request
                        .entity_secret_ciphertext(encrypt_entity_secret(
                            &self.public_key,
                            &self.circle_entity_secret,
                        )?)
                        .build()?,
                ),
            )
            .await?;
        Ok(response)
    }

    pub async fn sign_ethereum_message(
        &self,
        wallet_id: Uuid,
        message: String,
    ) -> Result<Web3Signature> {
        let response = self
            .sign_message(
                SignMessageRequestBuilder::default()
                    .wallet_id(wallet_id)
                    .message(message)
                    .encoded_by_hex(false),
            )
            .await?;
        Ok(response.signature)
    }

    pub async fn verify_ethereum_message(
        &self,
        wallet_address: Web3Address,
        message: String,
        signature: Web3Signature,
    ) -> Result<bool> {
        let formatted_message =
            format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message);
        let message_hash = keccak256(formatted_message.as_bytes());
        let recovered_address = recover(&message_hash, signature.get_r_s_slice(), 0)?;
        Ok(recovered_address == wallet_address.into())
    }

    // TODO: sign typed data
}
