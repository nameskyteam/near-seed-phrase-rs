/// Derive [`NearSecretKey`](crate::secret::NearSecretKey) from given seed phrase, password and derivation path.
/// Invalid seed phrase or derivation path will cause panic.
///
/// # Example
/// ```
/// use near_seed_phrase::{derive_key, ToEncodedKey};
///
/// let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
/// let secret_key = derive_key!(phrase);
///
/// assert_eq!(secret_key.to_encoded_key(), "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv");
/// assert_eq!(secret_key.to_public_key().to_encoded_key(), "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828");
/// ```
#[macro_export]
macro_rules! derive_key {
    ($phrase:expr) => {
        $crate::__derive_key!($phrase, "", $crate::NearDerivationPath::default())
    };
    ($phrase:expr, $password:expr) => {
        $crate::__derive_key!($phrase, $password, $crate::NearDerivationPath::default())
    };
    ($phrase:expr, $password:expr, $path:expr) => {
        $crate::__derive_key!($phrase, $password, $path)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_key {
    ($phrase:expr, $password:expr, $path:expr) => {
        $crate::derive_key(
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
        .expect("Failed to derive key")
    };
}
