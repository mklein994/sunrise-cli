use crate::Error;
use jiff::{Zoned, tz::TimeZone};
use sunrise::{DawnType, SolarDay, SolarEvent};

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
    for (name, event) in [
        (
            "Astronomical Dawn",
            SolarEvent::Dawn(DawnType::Astronomical),
        ),
        ("Nautical Dawn", SolarEvent::Dawn(DawnType::Nautical)),
        ("Civil Dawn", SolarEvent::Dawn(DawnType::Civil)),
        ("Sunrise", SolarEvent::Sunrise),
        ("Sunset", SolarEvent::Sunset),
        ("Civil Dusk", SolarEvent::Dusk(DawnType::Civil)),
        ("Nautical Dusk", SolarEvent::Dusk(DawnType::Nautical)),
        (
            "Astronomical Dusk",
            SolarEvent::Dusk(DawnType::Astronomical),
        ),
    ] {
        {
            let time = SolarDay::new(
                sunrise::Coordinates::new(coord.lat, coord.lon).unwrap(),
                today.date(),
            )
            .event_time(event);
            println!(
                "{name}:\t{time:?}",
                name = name,
                time = time.to_zoned(TimeZone::system())
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
