use crate::error::Error;
use bip39::Mnemonic;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// NEAR BIP39 seed phrase.
///
/// Supported number of words are 12, 15, 18, 21, and 24.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearSeedPhrase(pub(crate) Mnemonic);

impl NearSeedPhrase {
    /// Generate a new seed phrase.
    pub fn generate(word_count: usize) -> Result<Self, Error> {
        Ok(Mnemonic::generate(word_count).map(Self)?)
    }

    pub fn word_count(&self) -> usize {
        self.0.word_count()
    }

    pub fn to_word_list(&self) -> Vec<String> {
        self.0.word_iter().map(|word| word.to_string()).collect()
    }
}

impl NearSeedPhrase {
    #[doc(hidden)]
    pub fn parse<T>(&self) -> Result<&Self, Error> {
        Ok(self)
    }
}

impl FromStr for NearSeedPhrase {
    type Err = Error;

    fn from_str(phrase: &str) -> Result<Self, Self::Err> {
        Ok(phrase.parse().map(Self)?)
    }
}

impl TryFrom<String> for NearSeedPhrase {
    type Error = Error;

    fn try_from(phrase: String) -> Result<Self, Self::Error> {
        Ok(phrase.parse().map(Self)?)
    }
}

impl Display for NearSeedPhrase {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
