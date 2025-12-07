use crate::error::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NearMnemonic(pub(crate) bip39::Mnemonic);

impl NearMnemonic {
    #[doc(hidden)]
    pub fn parse<T>(&self) -> Result<&Self, Error> {
        Ok(self)
    }
}

#[cfg(feature = "rand")]
impl NearMnemonic {
    pub fn generate() -> Result<Self, Error> {
        Self::generate_of(12)
    }

    pub fn generate_of(word_count: usize) -> Result<Self, Error> {
        Ok(bip39::Mnemonic::generate(word_count).map(Self)?)
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

impl std::str::FromStr for NearMnemonic {
    type Err = Error;

    fn from_str(mnemonic: &str) -> Result<Self, Self::Err> {
        Ok(mnemonic.parse().map(Self)?)
    }
}

impl std::fmt::Display for NearMnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
