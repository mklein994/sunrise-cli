#[derive(Debug)]
pub enum Error {
    ParseFloat((String, std::num::ParseFloatError)),
    Missing,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloat((context, _)) => write!(f, "could not parse as float: {:?}", context),
            Self::Missing => write!(f, "latitude and longitude are required"),
        }
    }
}
