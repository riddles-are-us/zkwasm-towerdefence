use super::event::Event;
use super::object::Collector;
use super::object::Dropped;
use super::object::Monster;
use super::object::Object;
use super::object::Spawner;
use super::object::Tower;
use super::ERROR_INVENTORY_NOT_FOUND;
use super::ERROR_POSITION_OCCUPIED;
use crate::config::spawn_monster;
use crate::config::CONFIG;
use crate::config::GLOBAL;
use crate::config::SPWAN_INTERVAL;
use crate::config::UPGRADE_COST;
use crate::game::object::InventoryObject;
use crate::game::serialize::U64arraySerialize;
use crate::player::Owner;
use crate::player::TDPlayer;
use crate::tile::coordinate::Coordinate;
use crate::tile::coordinate::RectCoordinate;
use crate::tile::coordinate::RectDirection;
use crate::tile::map::Map;
use crate::tile::map::PositionedObject;
use crate::MERKLE_MAP;
use crate::settlement::{SettlementInfo, UpgradeInfo};
use core::slice::IterMut;
use std::borrow::Borrow;
use serde::Serialize;
use std::usize;
use zkwasm_rust_sdk::require;

extern "C" {
    pub fn wasm_trace_size() -> u64;
}

// The global state
#[derive(Clone, Serialize)]
pub struct State {
    #[serde(skip_serializing)]
    pub id_allocator: u64,
    pub counter: u64,
    pub map: Map<RectCoordinate>,
    pub monsters: Vec<PositionedObject<RectCoordinate, Monster>>,
    pub drops: Vec<PositionedObject<RectCoordinate, Dropped>>,
    pub collectors: Vec<PositionedObject<RectCoordinate, Collector>>,
    pub spawners: Vec<PositionedObject<RectCoordinate, Spawner>>,
    pub towers: Vec<PositionedObject<RectCoordinate, InventoryObject>>,
    pub events: Vec<Event>,
}

impl State {
    pub fn store(&self) {
        unsafe {
            let size = wasm_trace_size();
            zkwasm_rust_sdk::dbg!("store start is {}\n", size);
        }


        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut data = Vec::with_capacity(4096);
        data.append(&mut vec![
                self.id_allocator,
                self.counter,
                self.monsters.len() as u64,
                self.spawners.len() as u64,
                self.collectors.len() as u64,
                self.towers.len() as u64
            ]);

        self.monsters.iter().for_each(|x| x.to_u64_array(&mut data));
        self.spawners.iter().for_each(|x| x.to_u64_array(&mut data));
        self.collectors.iter().for_each(|x| x.to_u64_array(&mut data));
        self.towers.iter().for_each(|x| x.to_u64_array(&mut data));

        let ml = self.monsters.len();
        let tl = self.towers.len();
        zkwasm_rust_sdk::dbg!("towers: {}, monsters: {}\n", tl, ml);

        unsafe {
            let size = wasm_trace_size();
            zkwasm_rust_sdk::dbg!("store before kvpair is {}\n", size);
        }

        kvpair.set(&[0, 0, 0, 0], &data);
        unsafe {
            let size = wasm_trace_size();
            zkwasm_rust_sdk::dbg!("store end is {}\n", size);
        }

        //let root = kvpair.merkle.root;
        //zkwasm_rust_sdk::dbg!("after store: {:?}\n", root);
    }
    pub fn fetch(&mut self) -> bool {
        let kvpair = unsafe { &mut MERKLE_MAP };

        /*unsafe {
            let size = wasm_trace_size();
            zkwasm_rust_sdk::dbg!("kvpair start is {}\n", size);
        }*/

        let mut data = kvpair.get(&[0, 0, 0, 0]);
        /*unsafe {
            let datalen = data.len();
            let size = wasm_trace_size();
            zkwasm_rust_sdk::dbg!("kvpair end is {}\n", size);
            zkwasm_rust_sdk::dbg!("data len is {}\n", datalen);
        }*/

        if data.is_empty() {
            false
        } else {

            unsafe {
                let size = wasm_trace_size();
                zkwasm_rust_sdk::dbg!("fetch start is {}\n", size);
            }

            let mut data = data.iter_mut();
            //zkwasm_rust_sdk::dbg!("stored data: {:?}\n", data);
            self.id_allocator = *data.next().unwrap();
            self.counter = *data.next().unwrap();
            let monsters_len = *data.next().unwrap() as usize;
            let spawners_len = *data.next().unwrap() as usize;
            let collectors_len = *data.next().unwrap() as usize;
            let towers_len = *data.next().unwrap() as usize;
            //zkwasm_rust_sdk::dbg!("stored length: {} {} {}\n", monsters_len, spawners_len, towers_len);
            self.monsters = Vec::with_capacity(monsters_len);
            self.spawners = Vec::with_capacity(spawners_len);
            self.collectors = Vec::with_capacity(collectors_len);
            self.towers = Vec::with_capacity(towers_len);

            unsafe { self.monsters.set_len(monsters_len) };
            for i in 0..monsters_len {
                PositionedObject::<RectCoordinate, Monster>::modify_from_u64_array(&mut self.monsters[i], &mut data);
            }

            for _ in 0..spawners_len {
                let obj = PositionedObject::<RectCoordinate, Spawner>::from_u64_array(&mut data);
                self.map.set_occupy(&obj.position, 1);
                self.spawners.push(obj);
            }

            for _ in 0..collectors_len {
                let obj = PositionedObject::<RectCoordinate, Collector>::from_u64_array(&mut data);
                self.map.set_occupy(&obj.position, 1);
                self.collectors.push(obj);
            }

            unsafe { self.towers.set_len(towers_len) };
            for i in 0..towers_len {
                PositionedObject::<RectCoordinate, InventoryObject>::modify_from_u64_array(&mut self.towers[i], &mut data);
                self.map.set_occupy(&self.towers[i].position, 1);
            }

            unsafe {
                let size = wasm_trace_size();
                zkwasm_rust_sdk::dbg!("fetch end is {}\n", size);
            }
            true
        }
    }
    pub fn place_spawner_at(
        &mut self,
        object: Spawner,
        position: RectCoordinate,
    ) -> &PositionedObject<RectCoordinate, Spawner> {
        self.map.set_occupy(&position, 1);
        self.id_allocator += 1;
        self.spawners
            .push(PositionedObject::new(object, position, self.id_allocator));
        self.spawners.get(self.spawners.len() - 1).unwrap()
    }

    pub fn place_collector_at(
        &mut self,
        object: Collector,
        position: RectCoordinate,
    ) -> &PositionedObject<RectCoordinate, Collector> {
        self.map.set_occupy(&position, 1);
        self.id_allocator += 1;
        self.collectors
            .push(PositionedObject::new(object, position, self.id_allocator));
        self.collectors.get(self.collectors.len() - 1).unwrap()
    }

    pub fn place_tower_at(
        &mut self,
        object: &InventoryObject,
        position: RectCoordinate,
    ) -> Result<&PositionedObject<RectCoordinate, InventoryObject>, u32> {
        if self.map.get_occupy(&position) != 0 {
            Err(ERROR_POSITION_OCCUPIED)
        } else {
            self.id_allocator += 1;
            self.map.set_occupy(&position, 1);
            self.towers.push(PositionedObject::new(
                object.clone(),
                position,
                self.id_allocator,
            ));
            Ok(self.towers.get(self.towers.len() - 1).unwrap())
        }
    }

    pub fn remove_tower_at(
        &mut self,
        index: usize,
    ) -> PositionedObject<RectCoordinate, InventoryObject> {
        let tower = self.towers[index].clone();
        self.map.set_occupy(&tower.position, 0);
        self.towers.swap_remove(index)
    }

    pub fn spawn_monster_at(
        &mut self,
        object: Monster,
        position: RectCoordinate,
    ) -> &PositionedObject<RectCoordinate, Monster> {
        self.id_allocator += 1;
        self.monsters
            .push(PositionedObject::new(object, position, self.id_allocator));
        self.monsters.get(self.monsters.len() - 1).unwrap()
    }

    pub fn remove_monster(&mut self, index: usize) -> PositionedObject<RectCoordinate, Monster> {
        self.monsters.swap_remove(index)
    }

    pub fn spawn_dropped_at(
        &mut self,
        object: Dropped,
        position: RectCoordinate,
    ) -> &PositionedObject<RectCoordinate, Monster> {
        self.id_allocator += 1;
        self.drops
            .push(PositionedObject::new(object, position, self.id_allocator));
        self.monsters.get(self.monsters.len() - 1).unwrap()
    }

    pub fn remove_dropped(&mut self, index: usize) -> PositionedObject<RectCoordinate, Dropped> {
        self.drops.swap_remove(index)
    }

    pub fn spawn(&mut self, obj: PositionedObject<RectCoordinate, Object<RectDirection>>) {
        match obj.object {
            Object::Monster(m) => self.spawn_monster_at(m, obj.position),
            Object::Dropped(d) => self.spawn_dropped_at(d, obj.position),
            _ => unreachable!(),
        };
    }

    pub fn get_placed_inventory(&mut self, id: u64) -> Option<&mut InventoryObject> {
        for t in self.towers.iter_mut() {
            if t.object.object_id[0] == id {
                return Some(&mut t.object)
            }
        }
        None
    }
}

pub fn handle_place_tower(iid: &[u64; 4], pos: usize, feature: usize) -> Result<(), u32> {
    let global = unsafe { &mut crate::config::GLOBAL };
    let mut inventory_obj = InventoryObject::get(iid);
    if let Some(inventory_obj) = inventory_obj.as_mut() {
        let tower = inventory_obj.object.get_the_tower_mut();
        tower.direction = CONFIG.standard_towers[feature as usize].direction.clone();
        let position = global.map.coordinate_of_tile_index(pos);
        global.place_tower_at(inventory_obj, position)?;
        Ok(())
    } else {
        Err(ERROR_INVENTORY_NOT_FOUND)
    }
}

pub fn handle_update_inventory(iid: &[u64; 4], feature: u64, pid: &[u64; 2]) {
    let mut inventory_obj = InventoryObject::get(iid);
    if let Some(inventory_obj) = inventory_obj.as_mut() {
        let tower = inventory_obj.object.get_the_tower_mut();
        tower.owner[0] = pid[0];
        tower.owner[1] = pid[1];
        inventory_obj.store();
    } else {
        let mut tower = CONFIG.standard_towers[feature as usize].clone();
        tower.owner[0] = pid[0];
        tower.owner[1] = pid[1];
        let inventory_obj = InventoryObject::new(iid.clone(), Object::Tower(tower));
        inventory_obj.store();
    }
    let mut player_opt = TDPlayer::get_from_pid(pid);
    if let Some(player) = player_opt.as_mut() {
        if !player.owns(iid[0]) {
            player.data.inventory.push(iid[0]);
            player.store()
        }
    } else {
        let mut player = TDPlayer::new_from_pid(*pid);
        player.data.inventory.push(iid[0]);
        player.nonce = 1;
        player.store()
    }
}

pub fn handle_withdraw_tower(nonce: u64, iid: &[u64; 4], pkey: &[u64; 4]) {
    let inventory_obj = InventoryObject::get(iid);
    if inventory_obj.is_none() {
        unreachable!()
    } else {
        let obj = inventory_obj.unwrap().object.clone();
        let tower = obj.get_the_tower();
        unsafe {
            zkwasm_rust_sdk::require(tower.owner[0] == pkey[1]);
            zkwasm_rust_sdk::require(tower.owner[1] == pkey[2]);
        }

        let mut player = TDPlayer::get(pkey).unwrap();
        player.check_and_inc_nonce(nonce);
        let index_opt = player.data.inventory.iter().position(|&x| x == iid[0]);
        if let Some(index) = index_opt {
            player.data.inventory.swap_remove(index);
            player.store()
        } else {
            player.store()
        }
    }
}

pub fn handle_drop_tower(iid: &[u64; 4]) {
    let global = unsafe { &mut crate::config::GLOBAL };
    let mut inventory_obj = InventoryObject::get(iid).unwrap();
    let pos = global
        .towers
        .iter()
        .position(|x| x.object.object_id == *iid);
    if let Some(index) = pos {
        inventory_obj.object = global.towers[index].object.object.clone();
        inventory_obj.store();
        global.remove_tower_at(index);
    }
}

pub fn handle_collect_rewards(player: &mut TDPlayer, iid: &[u64; 4]) {
    //let inventory_obj = InventoryObject::get(iid);
    //let mut inventory_obj = InventoryObject::get(iid).unwrap();
    let cached_obj = unsafe {GLOBAL.get_placed_inventory(iid[0])};
    match cached_obj {
        Some(a) => {
            player.data.reward += a.reward;
            a.reward = 0u64;
        },
        None => {
        }
    }
    /*
    inventory_obj.reward = 0;
    inventory_obj.store();
    */
}

pub fn handle_upgrade_inventory(player: &mut TDPlayer, iid: &[u64; 4]) {
    //let global = unsafe { &mut crate::config::GLOBAL };
    let mut inventory_obj = InventoryObject::get(iid).unwrap();
    let tower = inventory_obj.object.get_the_tower_mut();
    let lvl = tower.lvl;
    zkwasm_rust_sdk::dbg!("check lvl ... {}\n", {tower.lvl} );
    unsafe { require(tower.lvl >= 1 && tower.lvl < 3) };
    let cost = UPGRADE_COST[(tower.lvl-1) as usize];
    zkwasm_rust_sdk::dbg!("check cost ... {}\n", {cost} );
    unsafe { require(player.data.reward >= cost) };
    player.data.reward -= cost;
    zkwasm_rust_sdk::dbg!("perform upgrade\n");
    inventory_obj.object.upgrade();
    let cached_obj = unsafe {GLOBAL.get_placed_inventory(iid[0])};
    match cached_obj {
        Some(a) => {
            a.object = inventory_obj.object.clone();
        },
        None => {
        }
    }
    UpgradeInfo::append(iid[0] as u32, lvl as u8);
    inventory_obj.store();
}

fn insert_into_sorted<T: Ord>(vec: &mut Vec<T>, element: T) {
    match vec.binary_search(&element) {
        Ok(_) => (),
        Err(pos) => vec.insert(pos, element),
    }
}

impl State {
    pub fn run(&mut self) {
        self.counter += 1;

        let mut collector = vec![];
        // figureout all the collectors in the state
        for obj in self.collectors.iter() {
            collector.push(obj.position.clone())
        }
        let mut termination_monster = vec![];
        let mut termination_drop = vec![];
        let mut spawn = vec![];

        for (index, obj) in self.monsters.iter_mut().enumerate() {
            //let m = &obj.object;
            if collector.contains(&obj.position) {
                insert_into_sorted(&mut termination_monster, index);
            } else {
                let index = self.map.index_of_tile_coordinate(&obj.position);
                let feature = self.map.get_feature(index);
                if let Some(f) = feature {
                    //unsafe { wasm_dbg(f.clone() as u64) };
                    obj.position = obj.position.adjacent(f)
                }
            }
        }

        for (index, obj) in self.drops.iter_mut().enumerate() {
            //let dropped = &obj.object;
            if collector.contains(&obj.position) {
                //terminates -= 1;
                termination_drop.push(index);
            } else {
                let index = self.map.index_of_tile_coordinate(&obj.position);
                let feature = self.map.get_feature(index);
                if let Some(f) = feature {
                    //unsafe { wasm_dbg(f.clone() as u64) };
                    obj.position = obj.position.adjacent(f)
                }
            }
        }

        for (_index, obj) in self.spawners.iter_mut().enumerate() {
            let spawner = &mut obj.object;
            if spawner.rate == 0 {
                spawner.count += 1;
                let monster = spawn_monster(spawner.count);
                let inner_obj = Object::Monster(monster);
                self.id_allocator += 1;
                spawn.push(PositionedObject::new(
                    inner_obj,
                    obj.position.clone(),
                    self.id_allocator,
                ));
                spawner.rate = SPWAN_INTERVAL;
            } else {
                spawner.rate -= 1
            }
            // TODO fill object spawner
        }


        const MAX_MAP_LEN: usize = 32;

        let mut x_position_mark: [([std::mem::MaybeUninit<usize>; MAX_MAP_LEN], usize); MAX_MAP_LEN] = [([std::mem::MaybeUninit::uninit(); MAX_MAP_LEN], 0); MAX_MAP_LEN];
        let mut y_position_mark: [([std::mem::MaybeUninit<usize>; MAX_MAP_LEN], usize); MAX_MAP_LEN] = [([std::mem::MaybeUninit::uninit(); MAX_MAP_LEN], 0); MAX_MAP_LEN];

        for (i, m) in self.monsters.iter().enumerate() {
            let (mx, my) = m.position.repr();
            let (arr, len) = &mut x_position_mark[mx as usize];
            arr[*len] = std::mem::MaybeUninit::new(i);
            *len += 1;
            let (arr, len) = &mut y_position_mark[my as usize];
            arr[*len] = std::mem::MaybeUninit::new(i);
            *len += 1;
        }

        let x_position_mark: &[([usize; MAX_MAP_LEN], usize); MAX_MAP_LEN] =
            unsafe { std::mem::transmute(&x_position_mark) };
        let y_position_mark: &[([usize; MAX_MAP_LEN], usize); MAX_MAP_LEN] =
            unsafe { std::mem::transmute(&y_position_mark) };

        let mut events = Vec::with_capacity(1024);
        for obj in self.towers.iter_mut() {
            let tower = obj.object.object.get_the_tower_mut();
                if tower.count == 0 {
                    let pos = &obj.position;
                    let (tx, ty) = pos.repr();
                    // Find the first monster according to the direction.
                    let mut monster_index = self.monsters.len();
                    let mut monster_dist = usize::MAX;
                    match tower.direction {
                        RectDirection::Top => {
                            for i in 0..x_position_mark[tx as usize].1 {
                                let m_index = x_position_mark[tx as usize].0[i];
                                let (_, my) = self.monsters[m_index as usize].position.repr();
                                if my < ty {
                                    let dist = (ty - my) as usize;
                                    if dist < monster_dist {
                                        monster_dist = dist;
                                        monster_index = m_index as usize;
                                    }
                                }
                            }
                        }
                        RectDirection::Bottom => {
                            for i in 0..x_position_mark[tx as usize].1 {
                                let m_index = x_position_mark[tx as usize].0[i];
                                let (_, my) = self.monsters[m_index as usize].position.repr();
                                if ty < my {
                                    let dist = (my - ty) as usize;
                                    if dist < monster_dist {
                                        monster_dist = dist;
                                        monster_index = m_index as usize;
                                    }
                                }
                            }
                        }
                        RectDirection::Right => {
                            for i in 0..y_position_mark[ty as usize].1 {
                                let m_index = y_position_mark[ty as usize].0[i];
                                let (mx, _) = self.monsters[m_index as usize].position.repr();
                                if tx < mx {
                                    let dist = (mx - tx) as usize;
                                    if dist < monster_dist {
                                        monster_dist = dist;
                                        monster_index = m_index as usize;
                                    }
                                }
                            }
                        }
                        RectDirection::Left => {
                            for i in 0..y_position_mark[ty as usize].1 {
                                let m_index = y_position_mark[ty as usize].0[i];
                                let (mx, _) = self.monsters[m_index as usize].position.repr();
                                if mx < tx {
                                    let dist = (tx - mx) as usize;
                                    if dist < monster_dist {
                                        monster_dist = dist;
                                        monster_index = m_index as usize;
                                    }
                                }
                            }
                        }
                    }
                    // Calculate damage and reward
                    if monster_index < self.monsters.len() {
                        let m_obj = &mut self.monsters[monster_index];
                        let m = &mut m_obj.object;
                        let hit_reward = m.hit;
                        if m.hp < tower.power {
                            m.hp = 0;
                        } else {
                            m.hp -= tower.power;
                        }
                        // Get reward
                        if m.hp == 0 {
                            //insert_into_sorted(&mut termination_monster, t.4);
                            obj.object.reward += m.kill; // kill reward
                            self.id_allocator += 1;
                            m.hp = m.born;
                            /* Disable drop feature
                            spawn.push(PositionedObject::new(
                                Object::Dropped(Dropped::new(10)),
                                self.monsters[t.4].position.clone(),
                                self.id_allocator,
                            ));
                            */
                        }
                        events.push(Event::Attack((tx, ty), m_obj.position.repr(), tower.power));
                        // Reset tower cooldown
                        tower.count = tower.cooldown;
                        obj.object.reward += hit_reward;
                    }
                } else {
                    tower.count -= 1;
                }
        }

        for idx in termination_monster.into_iter().rev() {
            self.remove_monster(idx);
        }

        /*
        termination_drop.reverse();
        for idx in termination_drop {
            self.remove_monster(idx);
        }
        */

        for obj in spawn.into_iter() {
            self.spawn(obj);
        }

        self.events = events;

        unsafe {
            let size = wasm_trace_size();
            zkwasm_rust_sdk::dbg!("end is {}\n", size);
        }
    }
}
