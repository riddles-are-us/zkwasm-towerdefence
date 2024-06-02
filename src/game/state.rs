use serde::Serialize;
use zkwasm_rust_sdk::wasm_dbg;
use super::event::Event;
use super::object::Dropped;
use super::object::Object;
use super::object::Tower;
use super::object::Monster;
use crate::tile::coordinate::Coordinate;
use crate::tile::coordinate::RectCoordinate;
use crate::tile::coordinate::RectDirection;
use crate::tile::map::Map;
//use crate::tile::coordinate::Tile;
use crate::tile::map::PositionedObject;
use crate::game::object::InventoryObject;
//use zkwasm_rust_sdk::require;

// The global state
#[derive(Clone, Serialize)]
pub struct State {
    pub treasure: u64,
    pub monston_spawn_counter: u64,
    pub hp: u64,
    pub map: Map<RectCoordinate, Object<RectDirection>>,
    pub events: Vec<Event>,
}

pub fn handle_place_tower(iid: &[u64; 4], pos: usize) {
    let global = unsafe { &mut crate::config::GLOBAL };
    let inventory_obj = InventoryObject::get(iid); 
    let position = global.map.coordinate_of_tile_index(pos);
    global
        .map
        .spawn_at(inventory_obj.unwrap().object, position);
}

pub fn handle_upgrade_inventory(iid: &[u64; 4]) {
    //let global = unsafe { &mut crate::config::GLOBAL };
    let mut inventory_obj = InventoryObject::get(iid).unwrap();
    let modifier = inventory_obj.upgrade_modifier;
    //let upgrade_cost = inventory_obj.cost * modifier;
    inventory_obj.upgrade_modifier = modifier * modifier;
    // unsafe { require(inventory_obj.cost <= global.treasure) };
    inventory_obj.cost *= 4;
    //global.treasure -= upgrade_cost;
    inventory_obj.object.upgrade();
    inventory_obj.store();
}

pub fn handle_run() {
    let global = unsafe { &mut crate::config::GLOBAL };
    let map = unsafe { &crate::config::GLOBAL.map };
    let objs = &mut global.map.objects;
    let mut collector = vec![];
    for obj in objs.iter() {
        if let Object::Collector(_) = obj.object.clone() {
            collector.push(obj.position.clone())
        }
    }
    let mut termination = vec![];
    let mut spawn = vec![];
    let mut tower_range: Vec<(Tower<RectDirection>, RectCoordinate, usize, usize, usize)> = vec![];

    let mut reward = 0;
    let mut damage = 0;
    //let mut terminates = global.terminates;
    //let mut monsters = global.monsters;

    for (index, obj) in objs.iter_mut().enumerate() {
        if let Object::Monster(m) = &mut obj.object {
            if collector.contains(&obj.position) {
                //terminates -= 1;
                termination.push(index);
                damage += m.hp;
            } else {
                let index = map.index_of_tile_coordinate(&obj.position);
                let feature = map.get_feature(index);
                if let Some(f) = feature {
                    unsafe { wasm_dbg(f.clone() as u64) };
                    obj.position = obj.position.adjacent(f)
                }
            }
        } else if let Object::Dropped(dropped) = &mut obj.object {
            if collector.contains(&obj.position) {
                reward += dropped.delta;
                //terminates -= 1;
                termination.push(index);
            } else {
                let index = map.index_of_tile_coordinate(&obj.position);
                let feature = map.get_feature(index);
                if let Some(f) = feature {
                    unsafe { wasm_dbg(f.clone() as u64) };
                    obj.position = obj.position.adjacent(f)
                }
            }
        } else if let Object::Spawner(spawner) = &mut obj.object {
            if spawner.count == 0 {
                spawn.push(PositionedObject::new(Object::Monster(Monster::new(10, 1, 1)), obj.position.clone()));
                spawner.count = spawner.rate
            } else {
                spawner.count -= 1
            }
            // TODO fill object spawner
        }
    }

    for (index, obj) in objs.iter_mut().enumerate() {
        if let Object::Tower(tower) = &mut obj.object {
            if tower.count == 0 {
                tower_range.push((
                    tower.clone(),
                    obj.position.clone(),
                    usize::max_value(),
                    index,
                    usize::max_value(),
                ));
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

    let mut events = vec![];

    for t in tower_range.iter_mut() {
        if t.4 != usize::max_value() {
            if let Object::Monster(m) = &mut objs[t.4].object {
                if m.hp < t.0.power {
                    m.hp = 0;
                } else {
                    m.hp -= t.0.power;
                }
                if m.hp == 0 {
                    termination.push(t.4);
                    spawn.push(PositionedObject::new(
                        Object::Dropped(Dropped::new(10)),
                        objs[t.4].position.clone(),
                    ));
                }
                events.push(Event::Attack(t.1.repr(), objs[t.4].position.repr(), 0))
            }
            if let Object::Tower(tower) = &mut objs[t.3].object {
                tower.count = tower.cooldown;
            }
        }
    }

    termination.reverse();
    for idx in termination {
        global.map.remove(idx);
    }

    //global.terminates = terminates;

    for obj in spawn.into_iter() {
        global.map.spawn(obj);
    }

    //global.monsters = monsters;

    if reward > damage {
        global.treasure += reward - damage;
    }

    global.events = events;
}
