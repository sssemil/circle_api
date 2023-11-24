use anyhow::Result;
use openssl::pkey::Public;
use openssl::rsa::{Padding, RsaRef};

pub fn encrypt_entity_secret(public_key: &RsaRef<Public>, entity_secret: &str) -> Result<String> {
    let entity_secret = hex::decode(entity_secret)?;
    let mut buffer = vec![0; public_key.size() as usize];
    public_key.public_encrypt(&entity_secret, &mut buffer, Padding::PKCS1_OAEP)?;
    Ok(base64::encode(buffer))
}
