/// Derive [`NearPrivateKey`](crate::private::NearPrivateKey) with given mnemonic, passphrase and derivation path.
///
/// # Example
/// ```
/// use near_seed_phrase::derive_key;
///
/// let mnemonic =
///     "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
/// let private_key = derive_key!(mnemonic);
///
/// assert_eq!(
///     private_key.to_string(),
///     "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv"
/// );
/// assert_eq!(
///     private_key.get_public_key().to_string(),
///     "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828"
/// );
/// ```
#[macro_export]
macro_rules! derive_key {
    ($mnemonic:expr) => {
        $crate::__derive_key!($mnemonic, "", $crate::NearDerivationPath::default())
    };
    ($mnemonic:expr, $passphrase:expr) => {
        $crate::__derive_key!(
            $mnemonic,
            $passphrase,
            $crate::NearDerivationPath::default()
        )
    };
    ($mnemonic:expr, $passphrase:expr, $path:expr) => {
        $crate::__derive_key!($mnemonic, $passphrase, $path)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_key {
    ($mnemonic:expr, $passphrase:expr, $path:expr) => {
        $crate::derive_key(
            std::borrow::Borrow::borrow(
                &$mnemonic
                    .parse::<$crate::NearMnemonic>()
                    .expect("failed to parse `NearMnemonic`"),
            ),
            $passphrase.as_ref(),
            std::borrow::Borrow::borrow(
                &$path
                    .parse::<$crate::NearDerivationPath>()
                    .expect("failed to parse `NearDerivationPath`"),
            ),
        )
        .expect("failed to derive key")
    };
}
