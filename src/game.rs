use crate::config::init_state;
use zkwasm_rust_sdk::require;
use zkwasm_rust_sdk::wasm_dbg;

pub mod event;
pub mod object;
pub mod state;
pub mod player;
pub mod serialize;

// This is a standalone game state manipulate module that connets with UI
// controllers and model handlers

const CMD_RUN: u64 = 0;
const CMD_PLACE_TOWER: u64 = 1;
const CMD_MINT_TOWER: u64 = 3;
const CMD_DROP_TOWER: u64 = 4;
const CMD_UPGRADE_TOWER: u64 = 5;
//const CMD_SPAWN: u64 = 3;

fn to_full_obj_id(id: u64) -> [u64; 4] {
    [id, 0xffff, 0xff01, 0xff02]
}

/// Step function receives a encoded command and changes the global state accordingly
pub fn handle_command(commands: &[u64; 4], pid: &[u64; 4]) {
    if commands[0] == CMD_RUN {
        unsafe {crate::config::GLOBAL.run()};
    } else if commands[0] == CMD_PLACE_TOWER {
        let player = player::Player::get(pid);
        let objindex = commands[1];
        unsafe { require(player.unwrap().owns(objindex)) };
        let pos = commands[2].to_le_bytes();
        let pos = u16::from_le_bytes(pos[0..2].try_into().unwrap());
        zkwasm_rust_sdk::dbg!("place tower ...\n");
        state::handle_place_tower(&to_full_obj_id(objindex), pos as usize);
    } else if commands[0] == CMD_UPGRADE_TOWER {
        let player = player::Player::get(pid);
        let objindex = commands[1];
        unsafe { require(player.unwrap().owns(objindex)) };
        state::handle_upgrade_inventory(&to_full_obj_id(objindex));
    } else if commands[0] == CMD_MINT_TOWER {
        let inventory_index = commands[1];
        unsafe {
            wasm_dbg(inventory_index as u64);
        }
        let feature = commands[2];
        state::handle_add_inventory(&to_full_obj_id(inventory_index), feature);
    } else if commands[0] == CMD_DROP_TOWER {
        let inventory_index = commands[1];
        unsafe {
            wasm_dbg(inventory_index as u64);
        }
        state::handle_drop_tower(&to_full_obj_id(inventory_index));
    }

}

pub struct State {}

impl State {
    pub fn get_state(pid: Vec<u64>) -> String {
        //zkwasm_rust_sdk::dbg!("finish loading {:?}", merkle_root);
        let global = unsafe { &crate::config::GLOBAL };
        serde_json::to_string(&global).unwrap()
    }
    pub fn initialize() {
        init_state()
    }
}

pub struct Transaction {
    pub command: [u64; 4],
}

impl Transaction {
    pub fn decode(params: [u64; 4]) -> Self {
        let command = [params[0], params[1], params[2], params[3]];
        Transaction {
            command,
        }
    }
    pub fn process(&self, pid: &[u64; 4]) -> bool {
        handle_command(&self.command, pid);
        true
    }
}
