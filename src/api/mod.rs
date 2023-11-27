mod signing;
mod token_lookup;
mod transactions;
mod wallet_sets;
mod wallets;

use crate::error::Result;
use reqwest::{Client, Method, Response};
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::Sha256;
use rsa::{Oaep, RsaPublicKey};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::CircleError;
use crate::models::public_key::PublicKeyResponse;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiSuccess<T> {
    data: T,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    code: i64,
    message: String,
}

pub struct CircleClient {
    base_url: String,
    api_key: String,
    circle_entity_secret: String,
    client: Client,
    public_key: RsaPublicKey,
}

impl CircleClient {
    pub async fn new(api_key: String, circle_entity_secret: String) -> Result<Self> {
        let base_url = "https://api.circle.com/v1/";
        let client = Client::new();

        let public_key = Self::fetch_public_key(&client, base_url, &api_key).await?;

        Ok(CircleClient {
            base_url: base_url.to_string(),
            api_key,
            circle_entity_secret,
            client,
            public_key,
        })
    }

    async fn fetch_public_key(
        client: &Client,
        base_url: &str,
        api_key: &str,
    ) -> Result<RsaPublicKey> {
        let url = format!("{}w3s/config/entity/publicKey", base_url);
        let res = client
            .get(&url)
            .header("Content-Type", "application/json")
            .bearer_auth(api_key)
            .send()
            .await?;

        let public_key_response: PublicKeyResponse = Self::parse_response(res).await?;
        let public_key =
            RsaPublicKey::from_public_key_pem(&public_key_response.public_key.replace("RSA ", ""))
                .unwrap();
        Ok(public_key)
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: String,
        body: Option<impl Serialize>,
    ) -> Result<T> {
        let request = self
            .client
            .request(method, &url)
            .bearer_auth(&self.api_key)
            .json(&body);

        let response = request.send().await?;
        Self::parse_response(response).await
    }

    async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T> {
        let request_id = response
            .headers()
            .get("X-Request-Id")
            .ok_or(CircleError::MissingRequestId)?;
        let request_id = request_id.to_str()?;
        let request_id = Uuid::parse_str(request_id)?;

        if response.status().is_success() {
            response
                .json::<ApiSuccess<T>>()
                .await
                .map(|res| res.data)
                .map_err(From::from)
        } else {
            let r = response.json::<ApiError>().await?;
            Err(CircleError::ApiError(request_id, r))?
        }
    }
}

pub fn encrypt_entity_secret(public_key: &RsaPublicKey, entity_secret: &str) -> Result<String> {
    let entity_secret = hex::decode(entity_secret)?;
    let padding = Oaep::new::<Sha256>();
    let enc_data = public_key.encrypt(&mut rand::thread_rng(), padding, &entity_secret[..])?;
    Ok(base64::encode(enc_data))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::wallet_set::CreateWalletSetResponse;

    const PUBLIC_RSA_KEY_STR: &str = "-----BEGIN RSA PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAxDiWHMTzDfIMeLVw4BGT\nOnhVv/jjccrcHFMtm0ShbOb8bu0b/hvtN2oEdWx2RTdNT7AvntB9R9vCv60lZrk0\nZtfR8p2lew++NKAfyEeqTfL8dpfjhPtTZWLjdKG9SzkN66SRXBz5fNae4qaDHG3N\nI8PtYmwRnpfy6VzpcdwOGQxv2nGmgT4AKD/A1wl+7W2KruUPlWaGRNsSiFVceNTR\nYWll5OsRM0BB9YLkwDAFm27e+XmISJlapSmD8Gqx3i5ZvpwINboj1JiEaqMe/bAs\nASYHR73qz7G/B9p7nSc6tKr3SToXivZqDC47NLa81JZuyHyc7U5r+pdcTXOCsa+T\nTS0Y+fEZZ5rOQO3nI3voDULvf1yDvWsJTJW8qi3RjtGlR3P3M0JwONF0xZUwtSal\nMOLWwNjZrC33LIuGoD4M+43/y62xkdXIE4CHXTo3annRPnktkdYxTVfIYUXH8JDA\ng7++dIE4ZaN41Eg2mWCt3SSry9BqrMhEcY7YyuVyzJnv59cGCi5sDnQHGlXs1xJG\n/5QSyhID9+J2RRtu4sZ+5aLIvcIkMsNhul0mbfTRr34f9MGqYv9mkuzHUC/ppykG\nOv1ZJ0PWMIX4WCMXLKSi5Ii4Eayrev4BZk6WtXnvgX+EY9j+/85o+XgvyaX1Z7hE\nPBYZ9E8aCK/7kzIK4tgXviECAwEAAQ==\n-----END RSA PUBLIC KEY-----\n";

    #[tokio::test]
    async fn test_rsa_import() {
        use rsa::pkcs8::DecodePublicKey;
        use rsa::sha2::Sha256;
        use rsa::{Oaep, RsaPublicKey};

        let public_key_str = PUBLIC_RSA_KEY_STR.clone().replace("RSA ", "");
        let public_key = RsaPublicKey::from_public_key_pem(&public_key_str).unwrap();

        // Encrypt
        let data = b"hello world";
        let padding = Oaep::new::<Sha256>();
        let enc_data = public_key
            .encrypt(&mut rand::thread_rng(), padding, &data[..])
            .expect("failed to encrypt");
        assert_ne!(&data[..], &enc_data[..]);
    }

    #[tokio::test]
    async fn test_encrypt_hex_entity_secret() {
        let public_key_str = PUBLIC_RSA_KEY_STR.clone().replace("RSA ", "");
        let public_key = RsaPublicKey::from_public_key_pem(&public_key_str).unwrap();
        let dummy_entity_secret = hex::encode("test");
        encrypt_entity_secret(&public_key, &dummy_entity_secret).unwrap();
    }

    #[tokio::test]
    async fn test_parse_wallet_set_response() {
        let json = "{\"data\":{\"walletSet\":{\"id\":\"0068d5a4-eb64-4399-8441-a9af33af80a0\",\"custodyType\":\"DEVELOPER\",\"name\":\"test_wallet_set\",\"updateDate\":\"2023-11-25T14:26:38Z\",\"createDate\":\"2023-11-25T14:26:38Z\"}}}";
        let wallet_set_response =
            serde_json::from_str::<ApiSuccess<CreateWalletSetResponse>>(json).unwrap();
        assert_eq!(wallet_set_response.data.wallet_set.name, "test_wallet_set");
    }
}
