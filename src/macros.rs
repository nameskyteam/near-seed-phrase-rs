/// Parse ed25519 secret key string from given seed phrase, password and derivation path.
/// Invalid seed phrase or derivation path will cause panic.
/// # Example
/// ```
/// use near_seed_phrase::secret;
///
/// let words = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
/// let secret = secret!(words);
/// assert_eq!(secret, "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv");
/// ```
#[macro_export]
macro_rules! secret {
    ($phrase:expr) => {{
        use $crate::{ToSecretKeyString, __keypair};

        let keypair = __keypair!($phrase, "", $crate::NearDerivationPath::default());
        keypair.to_secret_key_string()
    }};
    ($phrase:expr, $password:expr) => {{
        use $crate::{ToSecretKeyString, __keypair};

        let keypair = __keypair!($phrase, $password, $crate::NearDerivationPath::default());
        keypair.to_secret_key_string()
    }};
    ($phrase:expr, $password:expr, $path:expr) => {{
        use $crate::{ToSecretKeyString, __keypair};

        let keypair = __keypair!($phrase, $password, $path);
        keypair.to_secret_key_string()
    }};
}

/// Parse ed25519 public key string from given seed phrase, password and derivation path.
/// Invalid seed phrase or derivation path will cause panic.
/// # Example
/// ```
/// use near_seed_phrase::public;
///
/// let words = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
/// let public = public!(words);
/// assert_eq!(public, "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828");
/// ```
#[macro_export]
macro_rules! public {
    ($phrase:expr) => {{
        use $crate::{ToPublicKeyString, __keypair};

        let keypair = __keypair!($phrase, "", $crate::NearDerivationPath::default());
        keypair.to_public_key_string()
    }};
    ($phrase:expr, $password:expr) => {{
        use $crate::{ToPublicKeyString, __keypair};

        let keypair = __keypair!($phrase, $password, $crate::NearDerivationPath::default());
        keypair.to_public_key_string()
    }};
    ($phrase:expr, $password:expr, $path:expr) => {{
        use $crate::{ToPublicKeyString, __keypair};

        let keypair = __keypair!($phrase, $password, $path);
        keypair.to_public_key_string()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __keypair {
    ($phrase:expr, $password:expr, $path:expr) => {{
        use std::borrow::Borrow;
        use $crate::{derive_keypair, NearDerivationPath, NearSeedPhrase};

        derive_keypair(
            $phrase
                .parse::<NearSeedPhrase>()
                .expect("Failed to parse `NearSeedPhrase`")
                .borrow(),
            $password.as_ref(),
            $path
                .parse::<NearDerivationPath>()
                .expect("Failed to parse `NearDerivationPath`")
                .borrow(),
        )
        .expect("Failed to derive keypair")
    }};
}
