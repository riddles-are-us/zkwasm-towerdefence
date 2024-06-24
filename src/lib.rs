use zkwasm_rest_abi::*;
use wasm_bindgen::prelude::*;
pub mod config;
pub mod game;
pub mod settlement;
pub mod tile;
pub mod tx;

use crate::game::{Transaction, State};
use crate::config::Config;
zkwasm_rest_abi::create_zkwasm_apis!(Transaction, State, Config);
