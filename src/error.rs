#[derive(std::fmt::Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("invalid byte length")]
    InvalidByteLength,

    #[error("{0}")]
    Slip10Error(slipped10::Error),

    #[error(transparent)]
    Bip39Error(#[from] bip39::Error),

    #[error(transparent)]
    Ed25519SignatureError(#[from] ed25519_dalek::SignatureError),

    #[error(transparent)]
    Base58DecodeError(#[from] bs58::decode::Error),
}

impl From<slipped10::Error> for Error {
    fn from(e: slipped10::Error) -> Self {
        Self::Slip10Error(e)
    }
}
