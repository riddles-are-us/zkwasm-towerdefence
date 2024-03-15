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

// The global state
pub struct State {
    pub map: Map<RectCoordinate, Object<RectDirection>>,

}

pub static mut GLOBAL: State = State {
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
    let monster = Monster::new(10, 5, 1);
    let tower = Tower::new(5, 1, 3, 3, RectDirection::Top);
    let tower_left = Tower::new(5, 1, 4, 4, RectDirection::Left);
    let tower_right = Tower::new(5, 1, 4, 4, RectDirection::Right);
    let spawner = Spawner::new(4, 4);
    let spawner2 = Spawner::new(4, 4);
    let collector = Collector::new(5);
    let global = unsafe {&mut GLOBAL};
    for _ in 0..96 {
        global.map.tiles.push(
            Tile::new(RectCoordinate::new(0,0), None),
        )
    };

    global.map.set_feature(cor_to_index(0,0), Some(RectDirection::Bottom));
    global.map.set_feature(cor_to_index(0,1), Some(RectDirection::Right));
    global.map.set_feature(cor_to_index(1,1), Some(RectDirection::Right));
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

    global.map.spawn_at(Object::Monster(monster), RectCoordinate::new(0,0));
    global.map.spawn_at(Object::Spawner(spawner), RectCoordinate::new(0,0));
    global.map.spawn_at(Object::Spawner(spawner2), RectCoordinate::new(4,0));
    global.map.spawn_at(Object::Tower(tower.clone()), RectCoordinate::new(3,4));
    global.map.spawn_at(Object::Tower(tower_left), RectCoordinate::new(8,5));
    global.map.spawn_at(Object::Tower(tower_right.clone()), RectCoordinate::new(9,6));
    global.map.spawn_at(Object::Tower(tower_right.clone()), RectCoordinate::new(9,3));
    global.map.spawn_at(Object::Tower(tower_right), RectCoordinate::new(9,2));
    global.map.spawn_at(Object::Tower(tower), RectCoordinate::new(1,4));
    global.map.spawn_at(Object::Collector(collector), RectCoordinate::new(10,0));
}

pub fn handle_move(obj_index: usize, pos: usize) {
    let global = unsafe {&mut GLOBAL};
    let position = global.map.coordinate_of_tile_index(pos);
    let mut obj = global.map.objects.get_mut(obj_index).unwrap();
    obj.position = position;
    unsafe {
        wasm_dbg(obj.position.repr().0 as u64);
        wasm_dbg(obj.position.repr().1 as u64);
    }
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


    for (index, obj) in objs.iter_mut().enumerate() {
        if let Object::Monster(_) = &mut obj.object {
            if collector.contains(&obj.position) {
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
        else if let Object::Dropped(_) = &mut obj.object {
            if collector.contains(&obj.position) {
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
            if spawner.count == 0 {
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

    for obj in spawn.into_iter() {
        global.map.spawn(obj);
    }
}
