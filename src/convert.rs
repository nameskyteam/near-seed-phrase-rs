use crate::{AnyhowError, AnyhowResult};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, KEYPAIR_LENGTH, SECRET_KEY_LENGTH};

const ED25519_PREFIX: &str = "ed25519:";

/// Keypair that saving secret key and public key as string.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StringKeypair {
    pub secret: String,
    pub public: String,
}

pub fn keypair_to_string_keypair(keypair: &Keypair) -> StringKeypair {
    let secret = keypair.secret.to_encoded_key();
    let public = keypair.public.to_encoded_key();
    StringKeypair { secret, public }
}

pub trait ToEncodedKey {
    fn to_encoded_key(&self) -> String;
}

impl ToEncodedKey for SecretKey {
    /// Encode [`SecretKey`](ed25519_dalek::SecretKey) to secret key string.
    /// Note that the secret key string contains both secret key and public key.
    fn to_encoded_key(&self) -> String {
        let public = PublicKey::from(self);
        let mut bytes = [0; KEYPAIR_LENGTH];
        bytes[..SECRET_KEY_LENGTH].copy_from_slice(self.as_bytes());
        bytes[SECRET_KEY_LENGTH..].copy_from_slice(public.as_bytes());
        encode_key(&bytes)
    }
}

impl ToEncodedKey for PublicKey {
    /// Encode [`PublicKey`](ed25519_dalek::PublicKey) to public key string.
    fn to_encoded_key(&self) -> String {
        encode_key(self.as_bytes())
    }
}

fn encode_key(key: &[u8]) -> String {
    format!("{}{}", ED25519_PREFIX, bs58::encode(key).into_string())
}

pub trait FromEncodedKey: Sized {
    type Error;

    fn from_encoded_key(secret: &str) -> Result<Self, Self::Error>;
}

impl FromEncodedKey for SecretKey {
    type Error = AnyhowError;

    /// Decode secret key string to [`SecretKey`](ed25519_dalek::SecretKey).
    /// Note that the secret key string should contain both secret key and public key,
    /// public key should match secret key.
    fn from_encoded_key(secret: &str) -> Result<Self, Self::Error> {
        let bytes = decode_key(secret)?;
        if bytes.len() != KEYPAIR_LENGTH {
            anyhow::bail!("invalid secret key length");
        }
        let secret = SecretKey::from_bytes(&bytes[..SECRET_KEY_LENGTH])?;
        let public = PublicKey::from_bytes(&bytes[SECRET_KEY_LENGTH..])?;
        let public_from_secret = PublicKey::from(&secret);
        if public != public_from_secret {
            anyhow::bail!("public key doesn't match secret key");
        }
        Ok(secret)
    }
}

impl FromEncodedKey for PublicKey {
    type Error = AnyhowError;

    /// Decode public key string to [`PublicKey`](ed25519_dalek::PublicKey).
    fn from_encoded_key(public: &str) -> Result<Self, Self::Error> {
        let bytes = decode_key(public)?;
        Ok(PublicKey::from_bytes(&bytes)?)
    }
}

fn decode_key(key: &str) -> AnyhowResult<Vec<u8>> {
    let key = key.strip_prefix(ED25519_PREFIX).unwrap_or(key);
    Ok(bs58::decode(key).into_vec()?)
}
