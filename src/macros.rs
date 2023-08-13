/// Derive [`EncodedKeypair`](crate::EncodedKeypair) from given seed phrase, password and derivation path.
/// Invalid seed phrase or derivation path will cause panic.
///
/// # Example
/// ```
/// use near_seed_phrase::keypair;
///
/// let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
/// let keypair = keypair!(phrase);
///
/// assert_eq!(keypair.secret, "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv");
/// assert_eq!(keypair.public, "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828");
/// ```
#[macro_export]
macro_rules! keypair {
    ($phrase:expr) => {{
        let keypair = $crate::__keypair!($phrase, "", $crate::NearDerivationPath::default());
        $crate::encode_keypair(&keypair)
    }};
    ($phrase:expr, $password:expr) => {{
        let keypair = $crate::__keypair!($phrase, $password, $crate::NearDerivationPath::default());
        $crate::encode_keypair(&keypair)
    }};
    ($phrase:expr, $password:expr, $path:expr) => {{
        let keypair = $crate::__keypair!($phrase, $password, $path);
        $crate::encode_keypair(&keypair)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __keypair {
    ($phrase:expr, $password:expr, $path:expr) => {{
        $crate::derive_keypair(
            std::borrow::Borrow::borrow(
                &$phrase
                    .parse::<$crate::NearSeedPhrase>()
                    .expect("Failed to parse `NearSeedPhrase`"),
            ),
            $password.as_ref(),
            std::borrow::Borrow::borrow(
                &$path
                    .parse::<$crate::NearDerivationPath>()
                    .expect("Failed to parse `NearDerivationPath`"),
            ),
        )
        .expect("Failed to derive keypair")
    }};
}
