use crate::error::Error;
use crate::public::NearPublicKey;
use crate::ToEncodedKey;

#[derive(Clone, Debug)]
pub struct NearPrivateKey(pub(crate) ed25519_dalek::SigningKey);

#[cfg(feature = "rand")]
impl NearPrivateKey {
    pub fn generate() -> Self {
        Self(ed25519_dalek::SigningKey::generate(&mut rand::thread_rng()))
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
        Ok(Self(ed25519_dalek::SigningKey::from_bytes(&bytes)))
    }

    pub fn from_keypair_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let bytes: [u8; 64] = bytes.try_into().map_err(|_| Error::InvalidByteLength(64))?;
        Ok(ed25519_dalek::SigningKey::from_keypair_bytes(&bytes).map(Self)?)
    }

    pub fn get_public_key(&self) -> NearPublicKey {
        NearPublicKey(self.0.verifying_key())
    }
}

impl ed25519_dalek::Signer<ed25519_dalek::Signature> for NearPrivateKey {
    fn try_sign(
        &self,
        msg: &[u8],
    ) -> Result<ed25519_dalek::Signature, ed25519_dalek::SignatureError> {
        self.0.try_sign(msg)
    }
}

impl std::fmt::Display for NearPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_encoded_key())
    }
}
