use crate::error::Error;

const NEAR_DERIVATION_PATH_DEFAULT: &str = "m/44'/397'/0'";
const NEAR_DERIVATION_PATH_DEFAULT_LEDGER: &str = "m/44'/397'/0'/0'/1'";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NearDerivationPath(pub(crate) near_slip10::BIP32Path);

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

impl std::str::FromStr for NearDerivationPath {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(path.parse().map(Self)?)
    }
}

impl std::fmt::Display for NearDerivationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
