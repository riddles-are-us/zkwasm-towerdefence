use zkwasm_rust_sdk::require;
use crate::game::state::State;
use crate::tile::map::Map;
use crate::game::object::Spawner;
use crate::game::object::Collector;
//use crate::game::object::Dropped;
//use crate::tile::map::Map;
use crate::tile::coordinate::Tile;
use crate::tile::coordinate::RectDirection;
use crate::tile::coordinate::RectCoordinate;
use crate::tile::coordinate::Coordinate;
use crate::game::object::Tower;
use serde::Serialize;

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
    Tower::new(lvl, l[0], l[1], l[2], [0,0], dir)
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

#[derive (Serialize, Clone)]
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

const WIDTH:usize = 12;
const HEIGHT:usize= 12;

pub static mut GLOBAL: State = State {
    id_allocator: 0,
    monston_spawn_counter: 3,
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

fn cor_to_index(x: usize, y: usize) -> usize {
    x + y * WIDTH
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
    let spawner = Spawner::new(4, 4);
    let collector = Collector::new(5);
    for _ in 0..96 {
        global
            .map
            .tiles
            .push(Tile::new(RectCoordinate::new(0, 0), None))
    }
    let mut m = 0;
    em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m);
    em!(m); em!(m); mb!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mt!(m); em!(m);
    em!(m); em!(m); mb!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mt!(m); em!(m);
    em!(m); em!(m); mr!(m); mr!(m); mr!(m); mr!(m); mr!(m); mb!(m); em!(m); em!(m); mt!(m); em!(m);
    em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mb!(m); em!(m); em!(m); mt!(m); em!(m);
    em!(m); em!(m); em!(m); em!(m); mb!(m); ml!(m); ml!(m); ml!(m); em!(m); em!(m); mt!(m); em!(m);
    em!(m); em!(m); em!(m); em!(m); mb!(m); em!(m); em!(m); em!(m); em!(m); em!(m); mt!(m); em!(m);
    em!(m); em!(m); em!(m); em!(m); mr!(m); mr!(m); mr!(m); mr!(m); mr!(m); mr!(m); mt!(m); em!(m);

    if m != WIDTH * HEIGHT - 1 {
        unreachable!();
    }

    global
        .place_spawner_at(spawner, RectCoordinate::new(4, 0));
    global
        .place_collector_at(collector, RectCoordinate::new(10, 0));
}


