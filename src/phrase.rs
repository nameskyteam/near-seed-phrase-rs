use crate::errors::AnyhowError;
use crate::AnyhowResult;
use bip39::Mnemonic;

use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Word count of to be generated seed phrase.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WordCount {
    /// 12 words.
    W12,

    /// 15 words.
    W15,

    /// 18 words.
    W18,

    /// 21 words.
    W21,

    /// 24 words.
    W24,
}

impl WordCount {
    pub fn unwrap(&self) -> usize {
        match self {
            WordCount::W12 => 12,
            WordCount::W15 => 15,
            WordCount::W18 => 18,
            WordCount::W21 => 21,
            WordCount::W24 => 24,
        }
    }
}

/// NEAR BIP39 seed phrase.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NearSeedPhrase(pub Mnemonic);

impl NearSeedPhrase {
    /// Generate a new seed phrase.
    pub fn generate(word_count: WordCount) -> AnyhowResult<Self> {
        Ok(Self(Mnemonic::generate(word_count.unwrap())?))
    }

    /// Get an iterator over the words.
    pub fn word_iter(&self) -> impl Iterator<Item = &'static str> + Clone + '_ {
        self.0.word_iter()
    }
}

impl NearSeedPhrase {
    #[doc(hidden)]
    pub fn parse(&self) -> AnyhowResult<Self> {
        Ok(self.clone())
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
