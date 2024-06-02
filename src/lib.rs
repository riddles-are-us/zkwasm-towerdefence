use zkwasm_rest_abi::*;
use wasm_bindgen::prelude::*;
pub mod config;
pub mod game;
pub mod render;
pub mod tile;
pub mod tx;

use crate::game::{Transaction, State};
zkwasm_rest_abi::create_zkwasm_apis!(Transaction, State);
