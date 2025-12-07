mod encoding;
mod error;
mod macros;
mod mnemonic;
mod path;
mod private;
mod public;

pub use ed25519_dalek as ed25519;
pub use error::Error;
pub use mnemonic::NearMnemonic;
pub use path::NearDerivationPath;
pub use private::NearPrivateKey;
pub use public::NearPublicKey;

pub fn derive_key(
    mnemonic: &NearMnemonic,
    passphrase: &str,
    path: &NearDerivationPath,
) -> Result<NearPrivateKey, Error> {
    let key = slipped10::derive_key_from_path(
        &mnemonic.0.to_seed(passphrase),
        slipped10::Curve::Ed25519,
        &path.0,
    )?;
    NearPrivateKey::from_bytes(&key.key)
}

#[cfg(test)]
mod test {
    use crate::{derive_key, NearDerivationPath, NearMnemonic, NearPrivateKey, NearPublicKey};

    const MNEMONIC: &str =
        "fortune conduct light unusual gloom process wrap spare season exact anchor devote";

    const ENCODED_PRIVATE_KEY: &str =
        "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv";
    const ENCODED_PUBLIC_KEY: &str = "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828";

    #[test]
    fn test_derive_key() {
        let mnemonic = MNEMONIC.parse::<NearMnemonic>().unwrap();
        let private_key = derive_key(&mnemonic, "", &NearDerivationPath::default()).unwrap();

        assert_eq!(private_key.to_string(), ENCODED_PRIVATE_KEY);
        assert_eq!(private_key.get_public_key().to_string(), ENCODED_PUBLIC_KEY);
    }

    #[test]
    fn test_from_encoded_key() {
        let private_key: NearPrivateKey = ENCODED_PRIVATE_KEY.parse().unwrap();

        assert_eq!(private_key.to_string(), ENCODED_PRIVATE_KEY);
        assert_eq!(private_key.get_public_key().to_string(), ENCODED_PUBLIC_KEY);

        let public_key: NearPublicKey = ENCODED_PUBLIC_KEY.parse().unwrap();

        assert_eq!(public_key.to_string(), ENCODED_PUBLIC_KEY);
    }

    #[test]
    fn test_marco() {
        let private_key = derive_key!(MNEMONIC, "", NearDerivationPath::default());

        assert_eq!(private_key.to_string(), ENCODED_PRIVATE_KEY);
        assert_eq!(private_key.get_public_key().to_string(), ENCODED_PUBLIC_KEY);
    }
}
