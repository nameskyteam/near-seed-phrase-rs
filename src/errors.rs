use std::fmt::{Debug, Display};

pub const ERR_FAILED_TO_DERIVE_KEYPAIR: &str = "Failed to derive keypair";
pub const ERR_FAILED_TO_PARSE_NEAR_SEED_PHRASE: &str = "Failed to parse `NearSeedPhrase`";
pub const ERR_FAILED_TO_PARSE_NEAR_PATH: &str = "Failed to parse `NearPath`";

pub type AnyhowResult<T> = anyhow::Result<T>;

pub type AnyhowError = anyhow::Error;

pub trait IntoAnyhowError: Sized {
    fn into_anyhow_error(self) -> AnyhowError;
}

impl<T: Display + Debug + Send + Sync + 'static> IntoAnyhowError for T {
    /// Convert to [anyhow::Error](anyhow::Error)
    fn into_anyhow_error(self) -> AnyhowError {
        anyhow::anyhow!(self)
    }
}

// impl<T: std::error::Error> !IntoAnyhowError for T {}
