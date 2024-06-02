use zkwasm_rust_sdk::require;
use crate::game::state::State;
use crate::tile::map::Map;
use crate::game::object::Object;
use crate::game::object::Spawner;
use crate::game::object::Collector;
//use crate::game::object::Dropped;
//use crate::tile::map::Map;
use crate::tile::coordinate::Tile;
use crate::tile::coordinate::RectDirection;
use crate::tile::coordinate::RectCoordinate;
use crate::tile::coordinate::Coordinate;
use crate::game::object::Tower;

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

lazy_static::lazy_static! {
    pub static ref STANDARD_TOWER: [Tower<RectDirection>; 4] = [
        build_tower(1, RectDirection::Top),
        build_tower(1, RectDirection::Left),
        build_tower(1, RectDirection::Right),
        build_tower(1, RectDirection::Bottom)
    ];
}

pub static mut GLOBAL: State = State {
    treasure: 100,
    hp: 0,
    monston_spawn_counter: 3,
    map: Map {
        width: 12,
        height: 8,
        tiles: vec![],
        objects: vec![],
    },
    events: vec![],
};

fn cor_to_index(x: usize, y: usize) -> usize {
    x + y * 12
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

    global
        .map
        .set_feature(cor_to_index(2, 1), Some(RectDirection::Bottom));

    global
        .map
        .set_feature(cor_to_index(4, 0), Some(RectDirection::Bottom));
    global
        .map
        .set_feature(cor_to_index(4, 1), Some(RectDirection::Left));
    global
        .map
        .set_feature(cor_to_index(3, 1), Some(RectDirection::Left));

    global
        .map
        .set_feature(cor_to_index(2, 2), Some(RectDirection::Bottom));
    global
        .map
        .set_feature(cor_to_index(2, 3), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(3, 3), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(4, 3), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(5, 3), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(6, 3), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(7, 3), Some(RectDirection::Bottom));
    global
        .map
        .set_feature(cor_to_index(7, 4), Some(RectDirection::Bottom));
    global
        .map
        .set_feature(cor_to_index(7, 5), Some(RectDirection::Left));
    global
        .map
        .set_feature(cor_to_index(6, 5), Some(RectDirection::Left));
    global
        .map
        .set_feature(cor_to_index(5, 5), Some(RectDirection::Left));
    global
        .map
        .set_feature(cor_to_index(4, 5), Some(RectDirection::Left));
    global
        .map
        .set_feature(cor_to_index(3, 5), Some(RectDirection::Bottom));
    global
        .map
        .set_feature(cor_to_index(3, 6), Some(RectDirection::Bottom));
    global
        .map
        .set_feature(cor_to_index(3, 7), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(4, 7), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(5, 7), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(6, 7), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(7, 7), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(8, 7), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(9, 7), Some(RectDirection::Right));
    global
        .map
        .set_feature(cor_to_index(10, 7), Some(RectDirection::Top));
    global
        .map
        .set_feature(cor_to_index(10, 6), Some(RectDirection::Top));
    global
        .map
        .set_feature(cor_to_index(10, 5), Some(RectDirection::Top));
    global
        .map
        .set_feature(cor_to_index(10, 4), Some(RectDirection::Top));
    global
        .map
        .set_feature(cor_to_index(10, 3), Some(RectDirection::Top));
    global
        .map
        .set_feature(cor_to_index(10, 2), Some(RectDirection::Top));
    global
        .map
        .set_feature(cor_to_index(10, 1), Some(RectDirection::Top));

    global
        .map
        .spawn_at(Object::Spawner(spawner), RectCoordinate::new(4, 0));
    global
        .map
        .spawn_at(Object::Collector(collector), RectCoordinate::new(10, 0));
}


