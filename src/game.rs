use self::state::init_state;
use self::state::GLOBAL;
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::merkle::Merkle;
use zkwasm_rust_sdk::wasm_dbg;

pub mod event;
pub mod object;
pub mod state;

// This is a standalone game state manipulate module that connets with UI
// controllers and model handlers

static mut MERKLE: Merkle = Merkle { root: [0; 4] };

const CMD_RUN: u8 = 0;
const CMD_PLACE_TOWER: u8 = 1;
const CMD_UPGRADE_INVENTORY: u8 = 2;
const CMD_SPAWN: u8 = 3;

/// Step function receives a encoded command and changes the global state accordingly
#[wasm_bindgen]
pub fn step(command: u64) {
    let commands = command.to_le_bytes();
    unsafe {
        wasm_dbg(commands[0] as u64);
    };
    if commands[0] == CMD_RUN {
        state::handle_run();
    } else if commands[0] == CMD_PLACE_TOWER {
        let objindex = commands[1];
        unsafe {
            wasm_dbg(objindex as u64);
        }
        let pos = u16::from_le_bytes(commands[2..4].try_into().unwrap());
        unsafe {
            wasm_dbg(pos as u64);
        }
        state::handle_place_tower(objindex as usize, pos as usize);
    } else if commands[0] == CMD_UPGRADE_INVENTORY {
        let inventory_index = commands[1];
        unsafe {
            wasm_dbg(inventory_index as u64);
        }
        state::handle_upgrade_inventory(inventory_index as usize);
    }
}

// load the game with user account
#[wasm_bindgen]
pub fn load(account: u64, r0: u64, r1: u64, r2: u64, r3: u64) {
    unsafe {
        MERKLE.root = [r0, r1, r2, r3];
    }
}

#[wasm_bindgen]
pub fn init(seed: u64) {
    init_state()
    //zkwasm_rust_sdk::dbg!("finish loading {:?}", merkle_root);
}

#[wasm_bindgen]
pub fn get_state() -> String {
    //zkwasm_rust_sdk::dbg!("finish loading {:?}", merkle_root);
    let global = unsafe { &GLOBAL };
    serde_json::to_string(&global).unwrap()
}
