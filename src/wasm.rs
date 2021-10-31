use std::str::FromStr;
use sunrise::Azimuth;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
#[must_use]
pub fn get_sunrise_sunset(
    lat: f64,
    lon: f64,
    year: i32,
    month: u32,
    day: u32,
    azimuth: &str,
) -> Result<Vec<i64>, JsValue> {
    let (sunrise, sunset) = sunrise::time_of_transit(
        lat,
        lon,
        year,
        month,
        day,
        azimuth.parse::<AzimuthWasm>()?.0,
    );

    Ok(vec![sunrise, sunset])
}
