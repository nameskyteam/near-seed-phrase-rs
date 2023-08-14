use crate::error::Error;
use crate::public::NearPublicKey;
use crate::ToEncodedKey;
use ed25519_dalek::SigningKey;
use std::fmt::{Display, Formatter};

/// NEAR ed25519 secret key
#[derive(Clone, Debug)]
pub struct NearSecretKey(pub(crate) SigningKey);

impl NearSecretKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| Error::InvalidByteLength)?;
        Ok(Self(SigningKey::from_bytes(&bytes)))
    }

    pub fn to_keypair_bytes(&self) -> [u8; 64] {
        self.0.to_keypair_bytes()
    }

    pub fn from_keypair_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 64] = bytes.try_into().map_err(|_| Error::InvalidByteLength)?;
        Ok(SigningKey::from_keypair_bytes(&bytes).map(Self)?)
    }

    pub fn to_public_key(&self) -> NearPublicKey {
        NearPublicKey(self.0.verifying_key())
    }
}

impl Display for NearSecretKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_encoded_key())
    }
}
