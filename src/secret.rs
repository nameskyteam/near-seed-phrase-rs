use crate::error::Error;
use crate::public::NearPublicKey;
use crate::ToEncodedKey;
use ed25519_dalek::{Signature, SignatureError, Signer, SigningKey};
use std::fmt::{Display, Formatter};

/// NEAR ed25519 secret key
#[derive(Clone, Debug)]
pub struct NearSecretKey(pub(crate) SigningKey);

impl NearSecretKey {
    /// To public key
    pub fn to_public_key(&self) -> NearPublicKey {
        NearPublicKey(self.0.verifying_key())
    }
}

impl NearSecretKey {
    /// To raw bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    /// To raw keypair bytes
    pub fn to_keypair_bytes(&self) -> [u8; 64] {
        self.0.to_keypair_bytes()
    }

    /// From raw bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| Error::InvalidByteLength)?;
        Ok(Self(SigningKey::from_bytes(&bytes)))
    }

    /// From raw keypair bytes
    pub fn from_keypair_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 64] = bytes.try_into().map_err(|_| Error::InvalidByteLength)?;
        Ok(SigningKey::from_keypair_bytes(&bytes).map(Self)?)
    }
}

impl Signer<Signature> for NearSecretKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, SignatureError> {
        self.0.try_sign(msg)
    }
}

impl Display for NearSecretKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_encoded_key())
    }
}
