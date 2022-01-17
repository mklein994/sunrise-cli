use std::str::FromStr;
use sunrise::Azimuth as SunriseAzimuth;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Azimuth(SunriseAzimuth);

impl FromStr for Azimuth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Official" => Ok(Self(SunriseAzimuth::Official)),
            "Civil" => Ok(Self(SunriseAzimuth::Civil)),
            "Nautical" => Ok(Self(SunriseAzimuth::Nautical)),
            "Astronomical" => Ok(Self(SunriseAzimuth::Astronomical)),
            _ => Err(format!("Invalid azimuth: {:?}", s)),
        }
    }
}

impl From<Azimuth> for SunriseAzimuth {
    fn from(azimuth: Azimuth) -> Self {
        azimuth.0
    }
}

#[wasm_bindgen(js_name = "getSunriseSunset")]
#[must_use]
pub fn get_sunrise_sunset(
    lat: f64,
    lon: f64,
    year: i32,
    month: u32,
    day: u32,
    azimuth: &str,
) -> Result<Vec<i64>, JsValue> {
    let azimuth = azimuth.parse::<Azimuth>()?.into();
    let (sunrise, sunset) = sunrise::time_of_transit(lat, lon, year, month, day, azimuth);

    Ok(vec![sunrise, sunset])
}
