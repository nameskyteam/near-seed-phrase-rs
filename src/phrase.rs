use crate::errors::AnyhowError;
use bip39::Mnemonic;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearSeedPhrase(pub Mnemonic);

impl From<Mnemonic> for NearSeedPhrase {
    fn from(phrase: Mnemonic) -> Self {
        Self(phrase)
    }
}

impl FromStr for NearSeedPhrase {
    type Err = AnyhowError;

    fn from_str(phrase: &str) -> Result<Self, Self::Err> {
        Ok(phrase.parse().map(Self)?)
    }
}

impl TryFrom<&str> for NearSeedPhrase {
    type Error = AnyhowError;

    fn try_from(phrase: &str) -> Result<Self, Self::Error> {
        phrase.parse()
    }
}

impl TryFrom<String> for NearSeedPhrase {
    type Error = AnyhowError;

    fn try_from(phrase: String) -> Result<Self, Self::Error> {
        phrase.parse()
    }
}

impl Display for NearSeedPhrase {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
