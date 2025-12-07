use crate::error::Error;
use crate::private::NearPrivateKey;
use crate::public::NearPublicKey;

const ED25519_PREFIX: &str = "ed25519:";

pub trait ToEncodedKey {
    fn to_encoded_key(&self) -> String;
}

impl ToEncodedKey for NearPrivateKey {
    fn to_encoded_key(&self) -> String {
        encode_key(&self.to_keypair_bytes())
    }
}

impl ToEncodedKey for NearPublicKey {
    fn to_encoded_key(&self) -> String {
        encode_key(&self.to_bytes())
    }
}

pub trait FromEncodedKey: Sized {
    type Error;

    fn from_encoded_key(encoded_key: &str) -> Result<Self, Self::Error>;
}

impl FromEncodedKey for NearPrivateKey {
    type Error = Error;

    fn from_encoded_key(encoded_key: &str) -> Result<Self, Self::Error> {
        let bytes = decode_key(encoded_key)?;
        NearPrivateKey::from_keypair_bytes(&bytes)
    }
}

impl FromEncodedKey for NearPublicKey {
    type Error = Error;

    fn from_encoded_key(encoded_key: &str) -> Result<Self, Self::Error> {
        let bytes = decode_key(encoded_key)?;
        NearPublicKey::from_bytes(&bytes)
    }
}

fn encode_key(key: &[u8]) -> String {
    format!("{}{}", ED25519_PREFIX, bs58::encode(key).into_string())
}

fn decode_key(key: &str) -> Result<Vec<u8>, Error> {
    let key = key.strip_prefix(ED25519_PREFIX).unwrap_or(key);
    Ok(bs58::decode(key).into_vec()?)
}
