use crate::game::bigint_array_serializer;
use serde::Serialize;
use zkwasm_rest_abi::MERKLE_MAP;

#[derive(Clone, Serialize)]
pub struct Player {
    #[serde(skip_serializing)]
    pub player_id: [u64; 4],
    pub nonce: u64,
    #[serde(serialize_with = "bigint_array_serializer")]
    pub inventory: Vec<u64>,
}

impl Player {
    pub fn get(player_id: &[u64; 4]) -> Option<Self> {
        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut data = kvpair.get(&player_id);
        if data.is_empty() {
            None
        } else {
            let nonce = data.pop().unwrap();
            let player = Player {
                player_id: player_id.clone(),
                nonce,
                inventory: data,
            };
            Some(player)
        }
    }
    pub fn get_and_check_nonce(player_id: &[u64; 4], nonce: u64) -> Self {
        let player_opt = Player::get(player_id);
        match player_opt {
            None => {
                unsafe {zkwasm_rust_sdk::require(nonce == 0)};
                let player = Player {
                    nonce,
                    player_id: *player_id,
                    inventory: vec![],
                };
                player.store();
                player
            },
            Some (mut player) => {
                player.check_and_inc_nonce(nonce);
                player
            }
        }
    }
    pub fn store(&self) {
        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut c = self.inventory.clone();
        c.push(self.nonce);
        kvpair.set(&self.player_id, c.as_slice());
    }

    pub fn check_and_inc_nonce(&mut self, nonce: u64) {
        unsafe {zkwasm_rust_sdk::require(self.nonce == nonce)};
        self.nonce += 1;
    }

    pub fn owns(&self, tower_id: u64) -> bool {
        for o in self.inventory.iter() {
            if *o == tower_id {
                return true;
            }
        }
        return false;
    }
}
