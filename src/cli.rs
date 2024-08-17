use crate::Error;
use jiff::{tz::TimeZone, Timestamp, Zoned};
use sunrise::Azimuth;

/// Geographic coordinates
pub struct Coord {
    /// latitude
    lat: f64,

    /// longitude
    lon: f64,
}

impl Coord {
    pub fn try_new() -> Result<Self, Error> {
        if let [lat, lon, ..] = std::env::args()
            .skip(1)
            .take(2)
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()?[..]
        {
            Ok(Self { lat, lon })
        } else {
            Err(error::CliError::Missing.into())
        }
    }
}

pub fn run(coord: &Coord) {
    let today = Zoned::now();
    for (name, azimuth) in [
        ("Official", Azimuth::Official),
        ("Civil", Azimuth::Civil),
        ("Nautical", Azimuth::Nautical),
        ("Astronomical", Azimuth::Astronomical),
    ] {
        {
            let angle = azimuth.angle();
            let (sunrise, sunset) = sunrise::time_of_transit(
                coord.lat,
                coord.lon,
                today.year().into(),
                today.month().try_into().unwrap(),
                today.day().try_into().unwrap(),
                azimuth,
            );
            println!(
                "{name} ({angle:.3}\u{b0}):\t{sunrise:?}\t{sunset:?}",
                name = name,
                angle = angle,
                sunrise = Zoned::new(Timestamp::from_second(sunrise).unwrap(), TimeZone::system()),
                sunset = Zoned::new(Timestamp::from_second(sunset).unwrap(), TimeZone::system())
            );
        };
    }
}

pub mod error {
    use std::fmt;

    #[derive(Debug)]
    pub enum CliError {
        Missing,
    }

    impl fmt::Display for CliError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Missing => write!(f, "latitude and longitude are required"),
            }
        }
    }

    impl std::error::Error for CliError {}
}
