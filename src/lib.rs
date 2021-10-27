mod error;

use std::str::FromStr;

use chrono::prelude::*;
use error::Error;
use sunrise::Azimuth;
use wasm_bindgen::prelude::*;

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

struct AzimuthWasm(Azimuth);

impl FromStr for AzimuthWasm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Official" => Ok(Self(Azimuth::Official)),
            "Civil" => Ok(Self(Azimuth::Civil)),
            "Nautical" => Ok(Self(Azimuth::Nautical)),
            "Astronomical" => Ok(Self(Azimuth::Astronomical)),
            _ => Err(format!("Invalid azimuth: {:?}", s)),
        }
    }
}

#[wasm_bindgen]
pub fn get_sunrise_sunset(
    lat: f64,
    lon: f64,
    date: &str,
    azimuth: &str,
) -> Result<Vec<i64>, JsValue> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let (sunrise, sunset) = sunrise::time_of_transit(
        lat,
        lon,
        date.year(),
        date.month(),
        date.day(),
        azimuth.parse::<AzimuthWasm>()?.0,
    );

    Ok(vec![sunrise * 1000, sunset * 1000])
}
