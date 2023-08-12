use crate::AnyhowError;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, KEYPAIR_LENGTH, SECRET_KEY_LENGTH};

const ED25519_PREFIX: &str = "ed25519:";

pub trait ToStringSecretKey {
    fn to_string_secret_key(&self) -> String;
}

impl ToStringSecretKey for SecretKey {
    /// Convert ed25519 [`SecretKey`](ed25519_dalek::SecretKey) to string.
    /// Note that the secret key string not only contains private key but also contains public key.
    fn to_string_secret_key(&self) -> String {
        let public = PublicKey::from(self);
        let mut bytes = [0; KEYPAIR_LENGTH];
        bytes[..SECRET_KEY_LENGTH].copy_from_slice(self.as_bytes());
        bytes[SECRET_KEY_LENGTH..].copy_from_slice(public.as_bytes());
        format!("{}{}", ED25519_PREFIX, bs58::encode(&bytes).into_string())
    }
}

pub trait ToStringPublicKey {
    fn to_string_public_key(&self) -> String;
}

impl ToStringPublicKey for PublicKey {
    /// Convert ed25519 [`PublicKey`](ed25519_dalek::PublicKey) to string.
    fn to_string_public_key(&self) -> String {
        format!(
            "{}{}",
            ED25519_PREFIX,
            bs58::encode(self.as_bytes()).into_string()
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StringKeypair {
    pub secret: String,
    pub public: String,
}

pub trait ToStringKeypair {
    fn to_string_keypair(&self) -> StringKeypair;
}

impl ToStringKeypair for Keypair {
    /// Convert ed25519 [`Keypair`](ed25519_dalek::Keypair) to [`StringKeypair`].
    /// Note that the secret key string not only contains private key but also contains public key.
    fn to_string_keypair(&self) -> StringKeypair {
        StringKeypair {
            secret: self.secret.to_string_secret_key(),
            public: self.public.to_string_public_key(),
        }
    }
}

pub trait FromSecretKeyStr: Sized {
    type Error;

    fn from_secret_key_str(secret: &str) -> Result<Self, Self::Error>;
}

impl FromSecretKeyStr for SecretKey {
    type Error = AnyhowError;

    /// Convert ed25519 secret key string to [`SecretKey`](ed25519_dalek::SecretKey).
    /// Note that the secret key string should not only contains private key but also contains public key.
    fn from_secret_key_str(secret: &str) -> Result<Self, Self::Error> {
        let secret = if secret.to_lowercase().starts_with(ED25519_PREFIX) {
            &secret[ED25519_PREFIX.len()..]
        } else {
            secret
        };
        let bytes = bs58::decode(secret).into_vec()?;
        Ok(SecretKey::from_bytes(&bytes[..SECRET_KEY_LENGTH])?)
    }
}
