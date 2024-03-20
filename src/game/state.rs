use serde::Serialize;
use zkwasm_rust_sdk::require;
use zkwasm_rust_sdk::wasm_dbg;

use crate::tile::coordinate::Tile;
use crate::tile::map::Map;
use crate::tile::coordinate::RectCoordinate;
use crate::tile::coordinate::RectDirection;
use crate::tile::coordinate::Coordinate;
use crate::tile::map::PositionedObject;
use super::object::Collector;
use super::object::Monster;
use super::object::Dropped;
use super::object::Object;
use super::object::Spawner;
use super::object::Tower;

#[derive (Clone, Serialize)]
pub struct InventoryObject {
    pub used: bool,
    pub object: Object<RectDirection>,
}

impl InventoryObject {
    fn new(object: Object<RectDirection>) -> Self {
        Self {
            used: false,
            object,
        }
    }
}

const TOTAL_SPAWN:u64 = 20;

// The global state
#[derive (Clone, Serialize)]
pub struct State {
    pub inventory: Vec<InventoryObject>,
    pub treasure: u64,
    pub monsters: u64, // total monsters needs to spawn
    pub terminates: u64,
    pub hp: u64,
    pub map: Map<RectCoordinate, Object<RectDirection>>,

}

pub static mut GLOBAL: State = State {
    inventory: vec![],
    treasure: 0,
    hp: 0,
    monsters: TOTAL_SPAWN,
    terminates: TOTAL_SPAWN,
    map: Map {
        width: 12,
        height: 8,
        tiles: vec![],
        objects: vec![]
    }
};

fn cor_to_index(x:usize, y:usize) -> usize {
    x + y*12
}

pub fn init_state() {
    let global = unsafe {&mut GLOBAL};
    let tower = Tower::new(5, 1, 3, 3, RectDirection::Top);
    let tower_left = Tower::new(5, 1, 4, 4, RectDirection::Left);
    let tower_right = Tower::new(5, 1, 4, 4, RectDirection::Right);

    global.inventory = vec![
        InventoryObject::new(Object::Tower(tower.clone())),
        InventoryObject::new(Object::Tower(tower_left.clone())),
        InventoryObject::new(Object::Tower(tower_right.clone())),
    ];

    let monster = Monster::new(10, 5, 1);
    let spawner = Spawner::new(4, 4);
    let spawner2 = Spawner::new(4, 4);
    let collector = Collector::new(5);
    for _ in 0..96 {
        global.map.tiles.push(
            Tile::new(RectCoordinate::new(0,0), None),
        )
    };

    /*
    global.map.set_feature(cor_to_index(0,0), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(0,1), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(1,1), Some(RectDirection::Right));
    */
    global.map.set_feature(cor_to_index(2,1), Some(RectDirection::Bottom));


    global.map.set_feature(cor_to_index(4,0), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(4,1), Some(RectDirection::Left));
    global.map.set_feature(cor_to_index(3,1), Some(RectDirection::Left));



    global.map.set_feature(cor_to_index(2,2), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(2,3), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(3,3), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(4,3), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(5,3), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(6,3), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(7,3), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(7,4), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(7,5), Some(RectDirection::Left));
    global.map.set_feature(cor_to_index(6,5), Some(RectDirection::Left));
    global.map.set_feature(cor_to_index(5,5), Some(RectDirection::Left));
    global.map.set_feature(cor_to_index(4,5), Some(RectDirection::Left));
    global.map.set_feature(cor_to_index(3,5), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(3,6), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(3,7), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(4,7), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(5,7), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(6,7), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(7,7), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(8,7), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(9,7), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(10,7), Some(RectDirection::Top));
    global.map.set_feature(cor_to_index(10,6), Some(RectDirection::Top));
    global.map.set_feature(cor_to_index(10,5), Some(RectDirection::Top));
    global.map.set_feature(cor_to_index(10,4), Some(RectDirection::Top));
    global.map.set_feature(cor_to_index(10,3), Some(RectDirection::Top));
    global.map.set_feature(cor_to_index(10,2), Some(RectDirection::Top));
    global.map.set_feature(cor_to_index(10,1), Some(RectDirection::Top));

    //global.map.spawn_at(Object::Spawner(spawner), RectCoordinate::new(0,0));
    global.map.spawn_at(Object::Spawner(spawner2), RectCoordinate::new(4,0));
    global.map.spawn_at(Object::Collector(collector), RectCoordinate::new(10,0));
}

pub fn handle_place_tower(inventory_idx: usize, pos: usize) {
    let global = unsafe {&mut GLOBAL};
    unsafe {require(global.inventory[inventory_idx].used == false)};
    global.inventory[inventory_idx].used = true;
    let position = global.map.coordinate_of_tile_index(pos);
    global.map.spawn_at(global.inventory[inventory_idx].object.clone(), position);
}

pub fn handle_run() {
    let global = unsafe {&mut GLOBAL};
    let map = unsafe { &GLOBAL.map };
    let objs = &mut global.map.objects;
    let mut collector = vec![];
    for obj in objs.iter() {
        if let Object::Collector(_) = obj.object.clone() {
            collector.push(obj.position.clone())
        }
    }
    let mut termination = vec![];
    let mut spawn = vec![];
    let mut tower_range:Vec<(Tower<RectDirection>, RectCoordinate, usize, usize, usize)> = vec![];

    for (index, obj) in objs.iter_mut().enumerate() {
        if let Object::Tower(tower) = &mut obj.object {
            if tower.count == 0 {
                tower_range.push((tower.clone(), obj.position.clone(), usize::max_value(), index, usize::max_value()));
            } else {
                tower.count -= 1;
            }
        }
    }

    for (index, obj) in objs.iter_mut().enumerate() {
        if let Object::Monster(_) = &mut obj.object {
            for t in tower_range.iter_mut() {
                let range = t.0.range(&t.1, &obj.position);
                if range < t.2 {
                    t.2 = range;
                    t.4 = index;
                }
            }
        }
    }

    for t in tower_range.iter_mut() {
        if t.4 != usize::max_value() {
            if let Object::Monster(m) = &mut objs[t.4].object {
                if m.hp < 2 {
                    m.hp = 0;
                } else {
                    m.hp -= 2;
                }
                if m.hp == 0 {
                    termination.push(t.4);
                    spawn.push(PositionedObject::new(Object::Dropped(Dropped::new(10)), objs[t.4].position.clone()));
                }
            }
            if let Object::Tower(tower) = &mut objs[t.3].object {
                tower.count = tower.cooldown;
            }
        }
    }

    let mut reward = 0;
    let mut damage = 0;
    let mut terminates = global.terminates;
    let mut monsters = global.monsters;

    for (index, obj) in objs.iter_mut().enumerate() {
        if let Object::Monster(m) = &mut obj.object {
            if collector.contains(&obj.position) {
                terminates -= 1;
                termination.push(index);
                damage += m.hp;
            } else {
                let index = map.index_of_tile_coordinate(&obj.position);
                let feature = map.get_feature(index);
                if let Some(f) = feature {
                    unsafe {wasm_dbg(f.clone() as u64)};
                    obj.position = obj.position.adjacent(f)
                }
            }
        }
        else if let Object::Dropped(dropped) = &mut obj.object {
            if collector.contains(&obj.position) {
                reward += dropped.delta;
                termination.push(index);
            } else {
                let index = map.index_of_tile_coordinate(&obj.position);
                let feature = map.get_feature(index);
                if let Some(f) = feature {
                    unsafe {wasm_dbg(f.clone() as u64)};
                    obj.position = obj.position.adjacent(f)
                }
            }
        }

        else if let Object::Spawner(spawner) = &mut obj.object {
            if spawner.count == 0 && monsters > 0{
                monsters = monsters - 1;
                spawn.push(PositionedObject::new(Object::Monster(Monster::new(10, 1, 1)), obj.position.clone()));
                spawner.count = spawner.rate
            } else {
                spawner.count -= 1
            }
        }
    }
    termination.reverse();
    for idx in termination {
        global.map.remove(idx);
    }

    global.terminates = terminates;

    for obj in spawn.into_iter() {
        global.map.spawn(obj);
    }

    global.monsters = monsters;

    if reward > damage {
        global.treasure += reward - damage;
    }
}
