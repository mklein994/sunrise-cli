use chrono::prelude::*;

const USAGE: &str = concat!("Usage: ", env!("CARGO_BIN_NAME"), " <lat> <lon>");

#[derive(Debug)]
enum Error {
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

/// Geographic coordinates
struct Coord {
    /// latitude
    lat: f64,

    /// longitude
    lon: f64,
}

impl Coord {
    fn try_new() -> Result<Self, Error> {
        if let [lat, lon, ..] = std::env::args()
            .skip(1)
            .take(2)
            .map(|x| x.parse().map_err(|err| Error::ParseFloat((x, err))))
            .collect::<Result<Vec<_>, _>>()?[..]
        {
            Ok(Self { lat, lon })
        } else {
            Err(Error::Missing)
        }
    }
}

fn main() {
    let Coord { lat, lon } = Coord::try_new().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        eprintln!("{}", USAGE);
        std::process::exit(1);
    });

    let today = Local::today();
    let (sunrise, sunset) =
        sunrise::sunrise_sunset(lat, lon, today.year(), today.month(), today.day());

    println!(
        "{}\t{}",
        Local.timestamp(sunrise, 0).to_rfc3339(),
        Local.timestamp(sunset, 0).to_rfc3339()
    );
}
