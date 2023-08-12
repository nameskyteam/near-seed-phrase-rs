use std::fmt::{Debug, Display};

pub type AnyhowResult<T> = anyhow::Result<T>;

pub type AnyhowError = anyhow::Error;

pub trait IntoAnyhowError: Sized {
    fn into_anyhow_error(self) -> AnyhowError;
}

impl<T: Display + Debug + Send + Sync + 'static> IntoAnyhowError for T {
    /// Convert to [`anyhow::Error`].
    fn into_anyhow_error(self) -> AnyhowError {
        anyhow::anyhow!(self)
    }
}

// impl<T: std::error::Error> !IntoAnyhowError for T {}
