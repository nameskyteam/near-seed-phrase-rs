use crate::error::Error;
use crate::{NearSecretKey, ToEncodedKey};
use ed25519_dalek::{Signature, SignatureError, Verifier, VerifyingKey};
use std::fmt::{Display, Formatter};

/// NEAR ed25519 public key
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NearPublicKey(pub(crate) VerifyingKey);

impl NearPublicKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| Error::InvalidByteLength)?;
        Ok(VerifyingKey::from_bytes(&bytes).map(Self)?)
    }
}

impl Verifier<Signature> for NearPublicKey {
    fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), SignatureError> {
        self.0.verify(msg, signature)
    }
}

impl From<NearSecretKey> for NearPublicKey {
    fn from(secret_key: NearSecretKey) -> Self {
        secret_key.to_public_key()
    }
}

impl From<&NearSecretKey> for NearPublicKey {
    fn from(secret_key: &NearSecretKey) -> Self {
        secret_key.to_public_key()
    }
}

impl Display for NearPublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_encoded_key())
    }
}
