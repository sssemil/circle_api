use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    idempotency_key: Uuid,
    entity_secret_cipher_text: String,
}

impl Auth {
    pub fn new(idempotency_key: Uuid, entity_secret_cipher_text: String) -> Self {
        Auth {
            idempotency_key,
            entity_secret_cipher_text,
        }
    }
}