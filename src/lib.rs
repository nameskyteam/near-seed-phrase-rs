mod convert;
pub mod errors;
pub mod macros;
mod path;
mod phrase;

use crate::errors::{AnyhowError, AnyhowResult, IntoAnyhowError};
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use slip10::{derive_key_from_path, Curve};

pub use convert::{FromSecretKeyStr, StringKeypair, ToPublicKeyString, ToSecretKeyString};
pub use path::NearDerivationPath;
pub use phrase::{NearSeedPhrase, WordCount};

/// Derive ed25519 [`Keypair`](ed25519_dalek::Keypair) from given seed phrase, password and derivation path.
pub fn derive_keypair(
    phrase: &NearSeedPhrase,
    password: &str,
    path: &NearDerivationPath,
) -> AnyhowResult<Keypair> {
    let key = derive_key_from_path(&phrase.0.to_seed(password), Curve::Ed25519, &path.0)
        .map_err(IntoAnyhowError::into_anyhow_error)?;
    let secret = SecretKey::from_bytes(&key.key)?;
    let public = PublicKey::from(&secret);
    Ok(Keypair { secret, public })
}

#[cfg(test)]
mod test {
    use crate::{
        convert, derive_keypair, FromSecretKeyStr, NearDerivationPath, NearSeedPhrase,
        StringKeypair, ToPublicKeyString, ToSecretKeyString,
    };
    use ed25519_dalek::Keypair;

    const PHRASE: &str =
        "fortune conduct light unusual gloom process wrap spare season exact anchor devote";

    const SECRET_WITH_DEFAULT_PASSWORD_DEFAULT_PATH: &str =
        "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv";
    const PUBLIC_WITH_DEFAULT_PASSWORD_DEFAULT_PATH: &str =
        "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828";

    const TEST_PASSWORD: &str = "test password";
    const SECRET_WITH_TEST_PASSWORD_DEFAULT_PATH: &str =
        "ed25519:42tDSPEUPH7LJnExPTaonmZZNAgUFpqbL8M4UiRGiTjWsKsmwtMoNm5vZduudVMRyFKocYz1BWRak7772bP87EsE";
    const PUBLIC_WITH_TEST_PASSWORD_DEFAULT_PATH: &str =
        "ed25519:Gm7KTMDLfBDtgrD4bUnuHMfMnYmXSQLCZ24KVdVY7RRe";

    const SECRET_WITH_DEFAULT_PASSWORD_LEDGER_PATH: &str =
        "ed25519:2KCJTPWTZ5XkrbmgGTcZKkG4dM7i5TAxc1terb7YquHVr3HEfsCXbfp4pMLBsYCBbS1hBBsy6Pq6mHQQgSQZufRz";
    const PUBLIC_WITH_DEFAULT_PASSWORD_LEDGER_PATH: &str =
        "ed25519:EGHPmFXinZsN5h3XU3s4gPuaQ9n6QyaQtSpVHij1wyeG";

    #[test]
    fn test_derive_keypair() {
        let phrase = PHRASE.parse::<NearSeedPhrase>().unwrap();
        let keypair = derive_keypair(&phrase, "", &NearDerivationPath::default()).unwrap();
        assert_eq!(
            keypair.to_secret_key_string(),
            SECRET_WITH_DEFAULT_PASSWORD_DEFAULT_PATH
        );
        assert_eq!(
            keypair.to_public_key_string(),
            PUBLIC_WITH_DEFAULT_PASSWORD_DEFAULT_PATH
        );
    }

    #[test]
    fn test_from_secret_key_str() {
        let keypair =
            Keypair::from_secret_key_str(SECRET_WITH_DEFAULT_PASSWORD_DEFAULT_PATH).unwrap();
        assert_eq!(
            keypair.to_secret_key_string(),
            SECRET_WITH_DEFAULT_PASSWORD_DEFAULT_PATH
        );
        assert_eq!(
            keypair.to_public_key_string(),
            PUBLIC_WITH_DEFAULT_PASSWORD_DEFAULT_PATH
        );
    }

    #[test]
    fn test_macro_with_default_password_default_path() {
        let StringKeypair { secret, public } = convert!(PHRASE);
        assert_eq!(secret, SECRET_WITH_DEFAULT_PASSWORD_DEFAULT_PATH);
        assert_eq!(public, PUBLIC_WITH_DEFAULT_PASSWORD_DEFAULT_PATH);

        let StringKeypair { secret, public } = convert!(PHRASE, "");
        assert_eq!(secret, SECRET_WITH_DEFAULT_PASSWORD_DEFAULT_PATH);
        assert_eq!(public, PUBLIC_WITH_DEFAULT_PASSWORD_DEFAULT_PATH);

        let StringKeypair { secret, public } = convert!(PHRASE, "", NearDerivationPath::default());
        assert_eq!(secret, SECRET_WITH_DEFAULT_PASSWORD_DEFAULT_PATH);
        assert_eq!(public, PUBLIC_WITH_DEFAULT_PASSWORD_DEFAULT_PATH);
    }

    #[test]
    fn test_macros_with_test_password_default_path() {
        let StringKeypair { secret, public } =
            convert!(PHRASE, TEST_PASSWORD, NearDerivationPath::default());
        assert_eq!(secret, SECRET_WITH_TEST_PASSWORD_DEFAULT_PATH);
        assert_eq!(public, PUBLIC_WITH_TEST_PASSWORD_DEFAULT_PATH);
    }

    #[test]
    fn test_macros_with_default_password_ledger_path() {
        let StringKeypair { secret, public } = convert!(PHRASE, "", NearDerivationPath::ledger());
        assert_eq!(secret, SECRET_WITH_DEFAULT_PASSWORD_LEDGER_PATH);
        assert_eq!(public, PUBLIC_WITH_DEFAULT_PASSWORD_LEDGER_PATH);
    }
}
