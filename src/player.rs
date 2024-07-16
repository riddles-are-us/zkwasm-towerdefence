use crate::game::bigint_array_serializer;
use crate::StorageData;
use crate::Player;
use serde::Serialize;
use core::slice::IterMut;

#[derive(Clone, Serialize)]
pub struct PlayerData {
    #[serde(serialize_with = "bigint_array_serializer")]
    pub inventory: Vec<u64>,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            inventory: vec![],
        }
    }
}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let objects_size = *u64data.next().unwrap();
        let mut inventory = Vec::with_capacity(objects_size as usize);
        for _ in 0..objects_size {
            inventory.push(*u64data.next().unwrap());
        }
        PlayerData {
            inventory,
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.inventory.len() as u64);
        for c in self.inventory.iter() {
            data.push(*c as u64);
        }
    }
}

pub type TDPlayer = Player<PlayerData>;

pub trait Owner: Sized {
    fn owns(&self, tower_id: u64) -> bool;
    fn get(pkey: &[u64; 4]) -> Option<Self>;
}

impl Owner for TDPlayer {
    fn get(pkey: &[u64; 4]) -> Option<Self> {
        TDPlayer::get_from_pid(&TDPlayer::pkey_to_pid(pkey))
    }
    fn owns(&self, tower_id: u64) -> bool {
        for o in self.data.inventory.iter() {
            if *o == tower_id {
                return true;
            }
        }
        return false;
    }
}
