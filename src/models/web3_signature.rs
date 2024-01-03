use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, PartialEq)]
pub struct Web3Signature(([u8; 64], i32));

impl Web3Signature {
    pub fn get_r_s_slice(&self) -> &[u8; 64] {
        &self.0 .0
    }

    pub fn recovery_id(&self) -> i32 {
        self.0 .1
    }
}

impl fmt::Debug for Web3Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Web3Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sig = [0u8; 65];
        sig[..64].copy_from_slice(&self.0 .0);
        sig[64] = self.0 .1 as u8;
        write!(f, "0x{}", hex::encode(sig))
    }
}

impl Serialize for Web3Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let signature_str = self.to_string();
        serializer.serialize_str(&signature_str)
    }
}

struct SignatureVisitor;

impl<'de> Visitor<'de> for SignatureVisitor {
    type Value = Web3Signature;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a signature string in 0x format")
    }

    fn visit_str<E>(self, value: &str) -> Result<Web3Signature, E>
    where
        E: de::Error,
    {
        if value.len() != 132 || !value.starts_with("0x") {
            return Err(E::custom("Invalid signature format"));
        }

        let bytes = match hex::decode(&value[2..]) {
            Ok(bytes) => bytes,
            Err(_) => return Err(E::custom("Invalid hex in signature")),
        };

        if bytes.len() != 65 {
            return Err(E::custom("Invalid length of signature bytes"));
        }

        let mut arr = [0u8; 64];
        arr.copy_from_slice(&bytes[..64]);
        let int_val = bytes[64] as i32;

        Ok(Web3Signature((arr, int_val)))
    }
}

impl<'de> Deserialize<'de> for Web3Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SignatureVisitor)
    }
}

mod test {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct DummyObject {
        pub signature: Web3Signature,
    }

    #[test]
    fn test_signature_serialize_deserialize() {
        let random_bytes: Vec<u8> = (0..64).map(|_| rand::random::<u8>()).collect();
        let dummy_object = DummyObject {
            signature: Web3Signature((<[u8; 64]>::try_from(random_bytes).unwrap(), 42)),
        };
        let serialized = serde_json::to_string(&dummy_object).unwrap();
        let deserialized: DummyObject = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dummy_object.signature, deserialized.signature);
    }

    #[test]
    fn test_deserialize() {
        let signature_str = "\"0x36f761a1332fd670cd405d7488e8c840fdae6522cb51b6792e0ee16bdcf5d307621d144f57210b4cc67d873184e4cf109190a7e5e68d482ca22d285322389d3d1c\"";
        let signature: Web3Signature = serde_json::from_str(signature_str).unwrap();
        let serialized = serde_json::to_string(&signature).unwrap();
        assert_eq!(signature_str, serialized);
    }

    #[test]
    fn test_invalid_format_deserialize() {
        let invalid_format_str = "\"36f761a1332fd670cd405d7488e8c840fdae6522cb51b6792e0ee16bdcf5d307621d144f57210b4cc67d873184e4cf109190a7e5e68d482ca22d285322389d3d1c\"";
        let result: Result<Web3Signature, _> = serde_json::from_str(invalid_format_str);
        assert!(result.is_err(), "Expected an error for invalid format");
    }

    #[test]
    fn test_incorrect_length_deserialize() {
        let incorrect_length_str = "\"0x36f761\"";
        let result: Result<Web3Signature, _> = serde_json::from_str(incorrect_length_str);
        assert!(result.is_err(), "Expected an error for incorrect length");
    }

    #[test]
    fn test_invalid_hex_encoding_deserialize() {
        let invalid_hex_str = "\"0xGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG\"";
        let result: Result<Web3Signature, _> = serde_json::from_str(invalid_hex_str);
        assert!(
            result.is_err(),
            "Expected an error for invalid hex encoding"
        );
    }
}
