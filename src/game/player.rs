use serde::Serialize;
use zkwasm_rest_abi::MERKLE_MAP;

#[derive(Clone, Serialize)]
pub struct Player {
    pub player_id: [u64; 4],
    pub inventory: Vec<u64>,
}

impl Player {
    pub fn get(player_id: &[u64; 4]) -> Option<Self> {
        let kvpair = unsafe {&mut MERKLE_MAP};
        let data = kvpair.get(&player_id);
        if data.is_empty() {
            None
        } else {
            let player = Player {
                player_id: player_id.clone(),
                inventory: data,
            };
            Some(player)
        }
    }
    pub fn store(&self) {
        let kvpair = unsafe {&mut MERKLE_MAP};
        kvpair.set(&self.player_id, self.inventory.as_slice());
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
