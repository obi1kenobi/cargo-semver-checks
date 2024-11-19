/// An error that blocks further progress. "Terminal" in the sense of "cannot continue."
#[derive(Debug)]
pub(crate) enum TerminalError {
    WithAdvice(anyhow::Error, String),
    Other(anyhow::Error),
}

impl<E: std::error::Error + Send + Sync + 'static> From<E> for TerminalError {
    fn from(value: E) -> Self {
        Self::Other(value.into())
    }
}

pub(crate) trait IntoTerminalResult<T> {
    fn into_terminal_result(self) -> Result<T, TerminalError>;
}

impl<T> IntoTerminalResult<T> for Result<T, anyhow::Error> {
    fn into_terminal_result(self) -> Result<T, TerminalError> {
        self.map_err(TerminalError::Other)
    }
}
