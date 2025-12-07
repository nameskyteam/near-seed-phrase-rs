use crate::error::Error;
use crate::public::NearPublicKey;
use crate::ToEncodedKey;
use ed25519_dalek::{Signature, SignatureError, Signer, SigningKey};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct NearPrivateKey(pub(crate) SigningKey);

#[cfg(feature = "rand")]
impl NearPrivateKey {
    pub fn generate() -> Self {
        Self(SigningKey::generate(&mut rand::thread_rng()))
    }
}

impl NearPrivateKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn to_keypair_bytes(&self) -> [u8; 64] {
        self.0.to_keypair_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| Error::InvalidByteLength(32))?;
        Ok(Self(SigningKey::from_bytes(&bytes)))
    }

    pub fn from_keypair_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 64] = bytes.try_into().map_err(|_| Error::InvalidByteLength(64))?;
        Ok(SigningKey::from_keypair_bytes(&bytes).map(Self)?)
    }

    pub fn get_public_key(&self) -> NearPublicKey {
        NearPublicKey(self.0.verifying_key())
    }
}

impl Signer<Signature> for NearPrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, SignatureError> {
        self.0.try_sign(msg)
    }
}

impl Display for NearPrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_encoded_key())
    }
}
