#[derive(Debug)]
pub enum Error {
    ParseFloat(std::num::ParseFloatError),
    #[cfg(not(target_family = "wasm"))]
    Cli(crate::cli::error::CliError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloat(err) => err.fmt(f),
            #[cfg(not(target_family = "wasm"))]
            Self::Cli(err) => err.fmt(f),
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::ParseFloat(err)
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<crate::cli::error::CliError> for Error {
    fn from(err: crate::cli::error::CliError) -> Self {
        Self::Cli(err)
    }
}
