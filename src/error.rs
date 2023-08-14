#[derive(std::fmt::Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    Slip10Error(slip10::Error),

    #[error("{0}")]
    Bip39Error(#[from] bip39::Error),

    #[error("{0}")]
    Ed25519SignatureError(#[from] ed25519_dalek::SignatureError),

    #[error("{0}")]
    Base58DecodeError(#[from] bs58::decode::Error),

    #[error("invalid byte length")]
    InvalidByteLength,
}

impl From<slip10::Error> for Error {
    fn from(e: slip10::Error) -> Self {
        Self::Slip10Error(e)
    }
}
