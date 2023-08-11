use crate::errors::AnyhowError;
use crate::IntoAnyhowError;
use slip10::BIP32Path;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

const NEAR_DEFAULT_PATH: &str = "m/44'/397'/0'";
const NEAR_LEDGER_PATH: &str = "m/44'/397'/0'/0'/1'";

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearPath(pub BIP32Path);

impl NearPath {
    pub fn ledger() -> Self {
        Self(NEAR_LEDGER_PATH.parse().unwrap())
    }
}

impl Default for NearPath {
    fn default() -> Self {
        Self(NEAR_DEFAULT_PATH.parse().unwrap())
    }
}

impl From<BIP32Path> for NearPath {
    fn from(path: BIP32Path) -> Self {
        Self(path)
    }
}

impl FromStr for NearPath {
    type Err = AnyhowError;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        path.parse()
            .map(Self)
            .map_err(IntoAnyhowError::into_anyhow_error)
    }
}

impl TryFrom<&str> for NearPath {
    type Error = AnyhowError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        path.parse()
    }
}

impl TryFrom<String> for NearPath {
    type Error = AnyhowError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        path.parse()
    }
}

impl Display for NearPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
