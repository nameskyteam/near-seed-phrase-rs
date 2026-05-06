#[derive(std::fmt::Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("invalid byte length, expect {0}")]
    InvalidByteLength(usize),

    #[error("{0}")]
    Slip10Error(near_slip10::Error),

    #[error(transparent)]
    Bip39Error(#[from] bip39::Error),

    #[error(transparent)]
    Ed25519SignatureError(#[from] ed25519_dalek::SignatureError),

    #[error(transparent)]
    Base58DecodeError(#[from] bs58::decode::Error),
}

impl From<near_slip10::Error> for Error {
    fn from(e: near_slip10::Error) -> Self {
        Self::Slip10Error(e)
    }
}
