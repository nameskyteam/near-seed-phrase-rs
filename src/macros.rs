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
        let keypair = $crate::keypair!($phrase, "", $crate::NearPath::default());
        $crate::ToSecretKeyString::to_secret_key_string(&keypair)
    }};
    ($phrase:expr, $password:expr) => {{
        let keypair = $crate::keypair!($phrase, $password, $crate::NearPath::default());
        $crate::ToSecretKeyString::to_secret_key_string(&keypair)
    }};
    ($phrase:expr, $password:expr, $path:expr) => {{
        let keypair = $crate::keypair!($phrase, $password, $path);
        $crate::ToSecretKeyString::to_secret_key_string(&keypair)
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
        let keypair = $crate::keypair!($phrase, "", $crate::NearPath::default());
        $crate::ToPublicKeyString::to_public_key_string(&keypair)
    }};
    ($phrase:expr, $password:expr) => {{
        let keypair = $crate::keypair!($phrase, $password, $crate::NearPath::default());
        $crate::ToPublicKeyString::to_public_key_string(&keypair)
    }};
    ($phrase:expr, $password:expr, $path:expr) => {{
        let keypair = $crate::keypair!($phrase, $password, $path);
        $crate::ToPublicKeyString::to_public_key_string(&keypair)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! keypair {
    ($phrase:expr, $password:expr, $path:expr) => {{
        let phrase = $crate::NearSeedPhrase::try_from($phrase.clone())
            .expect($crate::errors::ERR_FAILED_TO_PARSE_NEAR_SEED_PHRASE);

        let path = $crate::NearPath::try_from($path.clone())
            .expect($crate::errors::ERR_FAILED_TO_PARSE_NEAR_PATH);

        $crate::derive_keypair(&phrase, $password, &path)
            .expect($crate::errors::ERR_FAILED_TO_DERIVE_KEYPAIR)
    }};
}
