use crate::error::Error;
use crate::public::NearPublicKey;
use crate::secret::NearSecretKey;

const ED25519_PREFIX: &str = "ed25519:";

pub trait ToEncodedKey {
    fn to_encoded_key(&self) -> String;
}

impl ToEncodedKey for NearSecretKey {
    /// Encode raw keypair bytes to NEAR secret key string. **NEAR use keypair as secret key**.
    fn to_encoded_key(&self) -> String {
        encode_key(&self.to_keypair_bytes())
    }
}

impl ToEncodedKey for NearPublicKey {
    /// Encode raw bytes to NEAR public key string.
    fn to_encoded_key(&self) -> String {
        encode_key(&self.to_bytes())
    }
}

pub trait FromEncodedKey: Sized {
    type Error;

    fn from_encoded_key(encoded_key: &str) -> Result<Self, Self::Error>;
}

impl FromEncodedKey for NearSecretKey {
    type Error = Error;

    /// Decode from NEAR secret key string (actually an encoded keypair string).
    fn from_encoded_key(encoded_key: &str) -> Result<Self, Self::Error> {
        let bytes = decode_key(encoded_key)?;
        NearSecretKey::from_keypair_bytes(&bytes)
    }
}

impl FromEncodedKey for NearPublicKey {
    type Error = Error;

    /// Decode from NEAR public key string.
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
