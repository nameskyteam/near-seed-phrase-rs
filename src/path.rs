use crate::error::Error;
use slipped10::BIP32Path;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

const NEAR_DERIVATION_PATH_DEFAULT: &str = "m/44'/397'/0'";
const NEAR_DERIVATION_PATH_DEFAULT_LEDGER: &str = "m/44'/397'/0'/0'/1'";

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearDerivationPath(pub(crate) BIP32Path);

impl NearDerivationPath {
    #[doc(hidden)]
    pub fn parse<T>(&self) -> Result<&Self, Error> {
        Ok(self)
    }
}

impl NearDerivationPath {
    pub fn default_ledger() -> Self {
        Self(NEAR_DERIVATION_PATH_DEFAULT_LEDGER.parse().unwrap())
    }
}

impl Default for NearDerivationPath {
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
