use crate::AnyhowError;
use ed25519_dalek::Keypair;

const ED25519_PREFIX: &str = "ed25519:";

pub trait ToSecretKeyString {
    fn to_secret_key_string(&self) -> String;
}

pub trait ToPublicKeyString {
    fn to_public_key_string(&self) -> String;
}

pub trait TryFromSecretKeyString: Sized {
    type Error;

    fn try_from_secret_key_string(secret: &str) -> Result<Self, Self::Error>;
}

impl ToSecretKeyString for Keypair {
    /// Convert [Keypair](ed25519_dalek::Keypair) to ed25519 secret key string.
    /// Note that the secret key not only contains private key but also contains public key
    fn to_secret_key_string(&self) -> String {
        format!(
            "{}{}",
            ED25519_PREFIX,
            bs58::encode(self.to_bytes()).into_string()
        )
    }
}

impl ToPublicKeyString for Keypair {
    /// Convert [Keypair](ed25519_dalek::Keypair) to ed25519 public key string
    fn to_public_key_string(&self) -> String {
        format!(
            "{}{}",
            ED25519_PREFIX,
            bs58::encode(self.public.to_bytes()).into_string()
        )
    }
}

impl TryFromSecretKeyString for Keypair {
    type Error = AnyhowError;

    /// Convert secret key string to [Keypair](ed25519_dalek::Keypair).
    /// Note that the secret key should not only contains private key but also contains public key
    fn try_from_secret_key_string(secret: &str) -> Result<Self, Self::Error> {
        let secret = if secret.to_lowercase().starts_with(ED25519_PREFIX) {
            &secret[ED25519_PREFIX.len()..]
        } else {
            secret
        };
        let bytes = bs58::decode(secret).into_vec()?;
        Ok(Keypair::from_bytes(&bytes)?)
    }
}
