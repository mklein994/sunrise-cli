mod error;

use chrono::prelude::*;
use error::Error;
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
            .map(|x| x.parse().map_err(|err| Error::ParseFloat((x, err))))
            .collect::<Result<Vec<_>, _>>()?[..]
        {
            Ok(Self { lat, lon })
        } else {
            Err(Error::Missing)
        }
    }
}

pub fn run(coord: Coord) {
    let today = Local::today();
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
                today.year(),
                today.month(),
                today.day(),
                azimuth,
            );
            println!(
                "{name} ({angle:.3}\u{b0}):\t{sunrise:?}\t{sunset:?}",
                name = name,
                angle = angle,
                sunrise = Local.timestamp(sunrise, 0),
                sunset = Local.timestamp(sunset, 0)
            );
        };
    }
}
