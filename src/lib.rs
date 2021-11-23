mod error;
use error::Error;

#[cfg(not(target_family = "wasm"))]
pub mod cli;

#[cfg(target_family = "wasm")]
mod wasm;
