use crate::error::Error;
use slip10::BIP32Path;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

const NEAR_DERIVATION_PATH_DEFAULT: &str = "m/44'/397'/0'";
const NEAR_DERIVATION_PATH_LEDGER: &str = "m/44'/397'/0'/0'/1'";

/// NEAR BIP32 derivation path.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearDerivationPath(pub(crate) BIP32Path);

impl NearDerivationPath {
    /// NEAR derivation path for Ledger.
    pub fn ledger() -> Self {
        Self(NEAR_DERIVATION_PATH_LEDGER.parse().unwrap())
    }
}

impl NearDerivationPath {
    #[doc(hidden)]
    pub fn parse<T>(&self) -> Result<&Self, Error> {
        Ok(self)
    }
}

impl Default for NearDerivationPath {
    /// NEAR derivation path by default.
    fn default() -> Self {
        Self(NEAR_DERIVATION_PATH_DEFAULT.parse().unwrap())
    }
}

impl FromStr for NearDerivationPath {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(path.parse().map(Self)?)
    }
}

impl TryFrom<String> for NearDerivationPath {
    type Error = Error;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        Ok(path.parse().map(Self)?)
    }
}

impl Display for NearDerivationPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
