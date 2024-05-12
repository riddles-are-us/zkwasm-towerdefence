use zkwasm_rust_sdk::require;

use crate::{game::object::Tower, tile::coordinate::RectDirection};

const TOWER_LEVEL: [[u64; 3]; 6] = [
    [5, 2, 4],
    [5, 3, 4],
    [5, 4, 4],
    [5, 5, 4],
    [5, 6, 3],
    [5, 7, 2],
];

pub fn build_tower(lvl: u64, dir: RectDirection) -> Tower<RectDirection> {
    let l = TOWER_LEVEL[lvl as usize];
    Tower::new(lvl, l[0], l[1], l[2], dir)
}

pub fn upgrade_tower(t: &mut Tower<RectDirection>) {
    unsafe { require(t.lvl < TOWER_LEVEL.len() as u64) };
    t.lvl = t.lvl + 1;
    let l = TOWER_LEVEL[t.lvl as usize];
    t.range = l[0];
    t.power = l[1];
    t.cooldown = l[2];
}

pub const UPGRADE_MODIFIER: u64 = 5;
pub const UPGRADE_COST_MODIFIER: u64 = 2;
