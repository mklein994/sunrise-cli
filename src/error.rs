#[derive(Debug)]
pub enum Error {
    ParseFloat(std::num::ParseFloatError),
    #[cfg(not(target_family = "wasm"))]
    Missing,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloat(err) => err.fmt(f),
            #[cfg(not(target_family = "wasm"))]
            Self::Missing => write!(f, "latitude and longitude are required"),
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::ParseFloat(err)
    }
}
