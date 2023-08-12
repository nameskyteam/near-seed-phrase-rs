mod convert;
pub mod errors;
pub mod macros;
mod path;
mod phrase;

use crate::errors::{AnyhowError, AnyhowResult, IntoAnyhowError};
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use slip10::{derive_key_from_path, Curve};

pub use convert::{FromSecretKeyStr, ToPublicKeyString, ToRef, ToSecretKeyString};
pub use path::NearPath;
pub use phrase::NearSeedPhrase;

/// Derive ed25519 keypair from given seed phrase, password and derivation path.
pub fn derive_keypair(
    phrase: &NearSeedPhrase,
    password: &str,
    path: &NearPath,
) -> AnyhowResult<Keypair> {
    let key = derive_key_from_path(&phrase.0.to_seed(password), Curve::Ed25519, &path.0)
        .map_err(IntoAnyhowError::into_anyhow_error)?;

    let secret = SecretKey::from_bytes(&key.key)?;
    let public = PublicKey::from(&secret);

    Ok(Keypair { secret, public })
}
