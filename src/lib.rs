use wasm_bindgen::prelude::*;
use zkwasm_rest_abi::*;
pub mod config;
pub mod game;
pub mod settlement;
pub mod tile;
pub mod tx;

use crate::config::Config;
use crate::game::{State, Transaction};
zkwasm_rest_abi::create_zkwasm_apis!(Transaction, State, Config);
