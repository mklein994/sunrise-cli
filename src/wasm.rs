use sunrise::{DawnType, SolarDay, SolarEvent};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Event(SolarEvent);

impl std::str::FromStr for Event {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dawn-astronomical" => Ok(Self(SolarEvent::Dawn(DawnType::Astronomical))),
            "dawn-nautical" => Ok(Self(SolarEvent::Dawn(DawnType::Nautical))),
            "dawn-civil" => Ok(Self(SolarEvent::Dawn(DawnType::Civil))),
            "sunrise" => Ok(Self(SolarEvent::Sunrise)),
            "sunset" => Ok(Self(SolarEvent::Sunset)),
            "dusk-civil" => Ok(Self(SolarEvent::Dusk(DawnType::Civil))),
            "dusk-nautical" => Ok(Self(SolarEvent::Dusk(DawnType::Nautical))),
            "dusk-astronomical" => Ok(Self(SolarEvent::Dusk(DawnType::Astronomical))),
            _ => todo!(),
        }
    }
}

#[wasm_bindgen(js_name = "getSolarEvents")]
pub fn get_solar_events(
    lat: f64,
    lon: f64,
    year: i16,
    month: i8,
    day: i8,
    events: Vec<String>,
) -> Result<Vec<i64>, JsValue> {
    let coords = sunrise::Coordinates::new(lat, lon).unwrap();
    let today = jiff::civil::Date::new(year, month, day).map_err(JsError::from)?;
    let mut times = vec![];
    for event in events {
        let event = event.parse::<Event>()?.0;
        let time = SolarDay::new(coords, today).event_time(event);
        times.push(time.as_second());
    }
    Ok(times)
}
