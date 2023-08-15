mod encoding;
mod error;
mod macros;
mod path;
mod phrase;
mod public;
mod secret;

pub use encoding::{FromEncodedKey, ToEncodedKey};
pub use error::Error;
pub use path::NearDerivationPath;
pub use phrase::NearSeedPhrase;
pub use public::NearPublicKey;
pub use secret::NearSecretKey;

pub use ed25519_dalek::{Signature, SignatureError, Signer, Verifier};

/// Derive [`NearSecretKey`](crate::secret::NearSecretKey) with given seed phrase, password and derivation path.
pub fn derive_key(
    phrase: &NearSeedPhrase,
    password: &str,
    path: &NearDerivationPath,
) -> Result<NearSecretKey, Error> {
    let key =
        slip10::derive_key_from_path(&phrase.0.to_seed(password), slip10::Curve::Ed25519, &path.0)?;
    NearSecretKey::from_bytes(&key.key)
}

#[cfg(test)]
mod test {
    use crate::{
        derive_key, FromEncodedKey, NearDerivationPath, NearPublicKey, NearSecretKey,
        NearSeedPhrase, ToEncodedKey,
    };

    const PHRASE: &str =
        "fortune conduct light unusual gloom process wrap spare season exact anchor devote";

    const ENCODED_SECRET_KEY: &str =
        "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv";
    const ENCODED_PUBLIC_KEY: &str = "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828";

    #[test]
    fn test_derive_key() {
        let phrase = PHRASE.parse::<NearSeedPhrase>().unwrap();
        let secret_key = derive_key(&phrase, "", &NearDerivationPath::default()).unwrap();

        assert_eq!(secret_key.to_encoded_key(), ENCODED_SECRET_KEY);
        assert_eq!(
            secret_key.to_public_key().to_encoded_key(),
            ENCODED_PUBLIC_KEY
        );
    }

    #[test]
    fn test_from_encoded_key() {
        let secret_key = NearSecretKey::from_encoded_key(ENCODED_SECRET_KEY).unwrap();

        assert_eq!(secret_key.to_encoded_key(), ENCODED_SECRET_KEY);
        assert_eq!(
            secret_key.to_public_key().to_encoded_key(),
            ENCODED_PUBLIC_KEY
        );

        let public_key = NearPublicKey::from_encoded_key(ENCODED_PUBLIC_KEY).unwrap();

        assert_eq!(public_key.to_encoded_key(), ENCODED_PUBLIC_KEY);
    }

    #[test]
    fn test_marco() {
        let secret_key = derive_key!(PHRASE, "", NearDerivationPath::default());

        assert_eq!(secret_key.to_encoded_key(), ENCODED_SECRET_KEY);
        assert_eq!(
            secret_key.to_public_key().to_encoded_key(),
            ENCODED_PUBLIC_KEY
        );
    }
}
