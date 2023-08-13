use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    Slip10(slip10::Error),

    #[error("{0}")]
    Bip39(#[from] bip39::Error),

    #[error("{0}")]
    Signature(#[from] ed25519_dalek::SignatureError),

    #[error("{0}")]
    Base58Decode(#[from] bs58::decode::Error),

    #[error("invalid secret key length")]
    InvalidSecretKeyLen,

    #[error("public key doesn't match secret key")]
    PublicKeyNotMatch,
}

impl From<slip10::Error> for Error {
    fn from(e: slip10::Error) -> Self {
        Self::Slip10(e)
    }
}
