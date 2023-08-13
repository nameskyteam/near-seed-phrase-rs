use crate::errors::AnyhowError;
use crate::AnyhowResult;
use bip39::Mnemonic;

use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// NEAR BIP39 seed phrase.
///
/// Supported number of words are 12, 15, 18, 21, and 24.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearSeedPhrase(pub Mnemonic);

impl NearSeedPhrase {
    /// Generate a new seed phrase.
    pub fn generate(word_count: usize) -> AnyhowResult<Self> {
        Ok(Self(Mnemonic::generate(word_count)?))
    }
}

impl NearSeedPhrase {
    /// Used in private macro [`__keypair!`](crate::__keypair).
    #[doc(hidden)]
    pub fn parse<T>(&self) -> AnyhowResult<&Self> {
        Ok(self)
    }
}

impl From<Mnemonic> for NearSeedPhrase {
    fn from(phrase: Mnemonic) -> Self {
        Self(phrase)
    }
}

impl FromStr for NearSeedPhrase {
    type Err = AnyhowError;

    fn from_str(phrase: &str) -> Result<Self, Self::Err> {
        Ok(Self(phrase.parse()?))
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
