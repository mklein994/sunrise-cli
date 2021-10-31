#![cfg(not(target_family = "wasm"))]

use sunrise_cli::cli::Coord;

const USAGE: &str = concat!("Usage: ", env!("CARGO_BIN_NAME"), " <lat> <lon>");

fn main() {
    let coord = Coord::try_new().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        eprintln!("{}", USAGE);
        std::process::exit(1);
    });

    sunrise_cli::cli::run(&coord);
}
