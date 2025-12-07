use crate::encoding::{decode_key, encode_key};
use crate::error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NearPublicKey(pub(crate) ed25519_dalek::VerifyingKey);

impl NearPublicKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| Error::InvalidByteLength(32))?;
        Ok(ed25519_dalek::VerifyingKey::from_bytes(&bytes).map(Self)?)
    }
}

impl std::str::FromStr for NearPublicKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = decode_key(s)?;
        NearPublicKey::from_bytes(&bytes)
    }
}

impl std::fmt::Display for NearPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&encode_key(&self.to_bytes()))
    }
}

impl ed25519_dalek::Verifier<ed25519_dalek::Signature> for NearPublicKey {
    fn verify(
        &self,
        msg: &[u8],
        signature: &ed25519_dalek::Signature,
    ) -> Result<(), ed25519_dalek::SignatureError> {
        self.0.verify(msg, signature)
    }
}
