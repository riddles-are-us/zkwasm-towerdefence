#![allow(unused_macros)]
use crate::game::object::Collector;
use crate::game::object::Monster;
use crate::game::object::Spawner;
use crate::game::state::State;
use crate::tile::map::Map;
use zkwasm_rust_sdk::require;
//use crate::game::object::Dropped;
//use crate::tile::map::Map;
use crate::game::object::Tower;
use crate::tile::coordinate::RectDirection;
use crate::tile::coordinate::Tile;
use serde::Serialize;

pub const SPWAN_INTERVAL:u64 = 3;

const MONSTER_LEVEL: [[u64; 3]; 3] = [
    [30, 1, 2],
    [30, 1, 10],
    [30, 1, 50],
];

const TOWER_LEVEL: [[u64; 3]; 3] = [
    [3, 1, 3],
    [5, 3, 2],
    [7, 10, 1],
];

pub const UPGRADE_COST: [u64; 2] = [1500, 8000];

pub fn spawn_monster(count: u64) -> Monster {
    if count % 10 == 0 {
        Monster::new(MONSTER_LEVEL[2][0], MONSTER_LEVEL[2][1], MONSTER_LEVEL[2][2])
    } else if count % 3 == 0 {
        Monster::new(MONSTER_LEVEL[1][0], MONSTER_LEVEL[1][1], MONSTER_LEVEL[1][2])
    } else {
        Monster::new(MONSTER_LEVEL[0][0], MONSTER_LEVEL[0][1], MONSTER_LEVEL[0][2])
    }
}

pub fn build_tower(lvl: u64, dir: RectDirection) -> Tower<RectDirection> {
    let l = TOWER_LEVEL[lvl as usize];
    Tower::new(lvl, l[0], l[1], l[2], [0, 0], dir)
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

#[derive(Serialize, Clone)]
pub struct Config {
    pub standard_towers: [Tower<RectDirection>; 4],
}

impl Config {
    pub fn to_json_string() -> String {
        serde_json::to_string(&CONFIG.clone()).unwrap()
    }
    pub fn flush_settlement() -> Vec<u8> {
        //SettleMentInfo::flush_settlement()
        vec![]
    }
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config {
        standard_towers: [
            build_tower(1, RectDirection::Top),
            build_tower(1, RectDirection::Left),
            build_tower(1, RectDirection::Right),
            build_tower(1, RectDirection::Bottom)
        ],
    };
}

const WIDTH: usize = 12;
const HEIGHT: usize = 8;

pub static mut GLOBAL: State = State {
    id_allocator: 0,
    map: Map {
        width: WIDTH,
        height: HEIGHT,
        tiles: vec![],
    },
    spawners: vec![],
    towers: vec![],
    collectors: vec![],
    drops: vec![],
    monsters: vec![],
    events: vec![],
};

pub fn cor_to_index(x: usize, y: usize) -> usize {
    x + y * WIDTH
}

macro_rules! pb {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        let spawner = Spawner::new(0, 3);
        let cor = global.map.coordinate_of_tile_index($idx);
        global.place_spawner_at(spawner, cor);
        global.map.set_feature($idx, Some(RectDirection::Bottom));
        $idx += 1;
    };
}

macro_rules! pt {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        let spawner = Spawner::new(0, 3);
        let cor = Map::coordinate_of_tile_index($idx);
        global.place_spawner_at(spawner, cor);
        global.map.set_feature($idx, Some(RectDirection::Top));
        $idx += 1;
    };
}

macro_rules! pl {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        let spawner = Spawner::new(0, 3);
        let cor = Map::coordinate_of_tile_index($idx);
        global.place_spawner_at(spawner, cor);
        global.map.set_feature($idx, Some(RectDirection::Left));
        $idx += 1;
    };
}

macro_rules! pr {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        let spawner = Spawner::new(0, 3);
        let cor = Map::coordinate_of_tile_index($idx);
        global.place_spawner_at(spawner, cor);
        global.map.set_feature($idx, Some(RectDirection::Right));
        $idx += 1;
    };
}


macro_rules! pc {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        let collector = Collector::new(5);
        let cor = global.map.coordinate_of_tile_index($idx);
        global.place_collector_at(collector, cor);
        $idx += 1;
    };
}



macro_rules! mb {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        global.map.set_feature($idx, Some(RectDirection::Bottom));
        $idx += 1;
    };
}

macro_rules! mt {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        global.map.set_feature($idx, Some(RectDirection::Top));
        $idx += 1;
    };
}

macro_rules! ml {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        global.map.set_feature($idx, Some(RectDirection::Left));
        $idx += 1;
    };
}

macro_rules! mr {
    ($idx: ident) => {
        let global = unsafe { &mut GLOBAL };
        global.map.set_feature($idx, Some(RectDirection::Right));
        $idx += 1;
    };
}

// empty
macro_rules! em {
    ($idx:ident) => {
        $idx += 1;
    };
}

pub fn init_state() {
    let global = unsafe { &mut GLOBAL };
    if global.map.tiles.is_empty() {
        for _ in 0..96 {
            global
                .map
                .tiles
                .push(Tile::new(None))
        }
        let mut m = 0;
        em!(m); em!(m); pb!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); pc!(m); em!(m);
        em!(m); em!(m); mb!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mt!(m); em!(m);
        em!(m); em!(m); mb!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mt!(m); em!(m);
        em!(m); em!(m); mr!(m); mr!(m); mr!(m); mr!(m); mr!(m); mb!(m); em!(m); em!(m); mt!(m); em!(m);
        em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mb!(m); em!(m); em!(m); mt!(m); em!(m);
        em!(m); em!(m); em!(m); em!(m); mb!(m); ml!(m); ml!(m); ml!(m); em!(m); em!(m); mt!(m); em!(m);
        em!(m); em!(m); em!(m); em!(m); mb!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mt!(m); em!(m);
        em!(m); em!(m); em!(m); em!(m); mr!(m); mr!(m); mr!(m); mr!(m); mr!(m); mr!(m); mt!(m); em!(m);
        //zkwasm_rust_sdk::dbg!("m is {}\n", m);

        if m != WIDTH * HEIGHT {
            unreachable!();
        }
    }
}
