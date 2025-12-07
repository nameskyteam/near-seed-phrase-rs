use crate::error::Error;
use bip39::Mnemonic;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearMnemonic(pub(crate) Mnemonic);

#[cfg(feature = "rand")]
impl NearMnemonic {
    pub fn generate() -> Result<Self, Error> {
        Self::generate_of(12)
    }

    pub fn generate_of(word_count: usize) -> Result<Self, Error> {
        Ok(Mnemonic::generate(word_count).map(Self)?)
    }
}

impl NearMnemonic {
    pub fn word_count(&self) -> usize {
        self.0.word_count()
    }

    pub fn words(&self) -> Vec<String> {
        self.0.words().map(|word| word.to_string()).collect()
    }
}

#[doc(hidden)]
impl NearMnemonic {
    pub fn parse<T>(&self) -> Result<&Self, Error> {
        Ok(self)
    }
}

impl FromStr for NearMnemonic {
    type Err = Error;

    fn from_str(mnemonic: &str) -> Result<Self, Self::Err> {
        Ok(mnemonic.parse().map(Self)?)
    }
}

impl TryFrom<String> for NearMnemonic {
    type Error = Error;

    fn try_from(mnemonic: String) -> Result<Self, Self::Error> {
        Ok(mnemonic.parse().map(Self)?)
    }
}

impl Display for NearMnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
