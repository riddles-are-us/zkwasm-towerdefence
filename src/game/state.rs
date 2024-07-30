use super::event::Event;
use super::object::Collector;
use super::object::Dropped;
use super::object::Monster;
use super::object::Object;
use super::object::Spawner;
use super::object::Tower;
use super::ERROR_POSITION_OCCUPIED;
use crate::player::TDPlayer;
use crate::player::Owner;
use crate::config::spawn_monster;
use crate::config::CONFIG;
use crate::config::SPWAN_INTERVAL;
use crate::config::UPGRADE_COST;
use crate::game::object::InventoryObject;
use crate::tile::coordinate::Coordinate;
use crate::tile::coordinate::RectCoordinate;
use crate::tile::coordinate::RectDirection;
use crate::tile::map::Map;
use crate::tile::map::PositionedObject;
use serde::Serialize;
use zkwasm_rust_sdk::require;
use crate::MERKLE_MAP;
use crate::game::serialize::U64arraySerialize;
use core::slice::IterMut;

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
        let kvpair = unsafe { &mut MERKLE_MAP };
        let monsters_data = self.monsters.iter().map(|x| x.to_u64_array()).flatten().collect::<Vec<u64>>();
        let spawners_data = self.spawners.iter().map(|x| x.to_u64_array()).flatten().collect::<Vec<u64>>();
        let collectors_data = self.collectors.iter().map(|x| x.to_u64_array()).flatten().collect::<Vec<u64>>();
        let towers_data = self.towers.iter().map(|x| x.to_u64_array()).flatten().collect::<Vec<u64>>();
        let data = vec![
            vec![
                self.id_allocator,
                self.counter,
                self.monsters.len() as u64,
                self.spawners.len() as u64,
                self.collectors.len() as u64,
                self.towers.len() as u64
            ], monsters_data, spawners_data, collectors_data, towers_data]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        //zkwasm_rust_sdk::dbg!("stored data: {:?}\n", data);
        let splen = self.spawners.len();
        //zkwasm_rust_sdk::dbg!("spawners: {}\n", splen);
        let mlen = self.monsters.len();
        //zkwasm_rust_sdk::dbg!("monsters: {}\n", mlen);
        kvpair.set(&[0,0,0,0], &data);
        let root = kvpair.merkle.root;
        //zkwasm_rust_sdk::dbg!("after store: {:?}\n", root);
    }
    pub fn fetch(&mut self) -> bool {
        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut data = kvpair.get(&[0,0,0,0]);
        if data.is_empty() {
            false
        } else {
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
            for _ in 0..monsters_len {
                let obj = PositionedObject::<RectCoordinate, Monster>::from_u64_array(&mut data);
                self.monsters.push(obj);
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
            for _ in 0..towers_len {
                let obj = PositionedObject::<RectCoordinate, InventoryObject>::from_u64_array(&mut data);
                self.map.set_occupy(&obj.position, 1);
                self.towers.push(obj);
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
        object: InventoryObject,
        position: RectCoordinate,
    ) -> Result<&PositionedObject<RectCoordinate, InventoryObject>, u32> {
        if self.map.get_occupy(&position) != 0 {
            Err(ERROR_POSITION_OCCUPIED)
        } else {
            self.id_allocator += 1;
            self.map.set_occupy(&position, 1);
            self.towers
                .push(PositionedObject::new(object, position, self.id_allocator));
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
}

pub fn handle_place_tower(iid: &[u64; 4], pos: usize) -> Result<(), u32> {
    let global = unsafe { &mut crate::config::GLOBAL };
    let inventory_obj = InventoryObject::get(iid);
    let position = global.map.coordinate_of_tile_index(pos);
    global.place_tower_at(inventory_obj.unwrap(), position)?;
    Ok(())
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
    //let inventory_obj = InventoryObject::get(iid);
    let pos = global
        .towers
        .iter()
        .position(|x| x.object.object_id == *iid);
    if let Some(index) = pos {
        global.remove_tower_at(index);
    }
}

pub fn handle_collect_rewards(player: &mut TDPlayer, iid: &[u64; 4]) {
    //let inventory_obj = InventoryObject::get(iid);
    let mut inventory_obj = InventoryObject::get(iid).unwrap();
    player.data.reward += inventory_obj.reward;
    inventory_obj.reward = 0;
    inventory_obj.store();
}



pub fn handle_upgrade_inventory(iid: &[u64; 4]) {
    //let global = unsafe { &mut crate::config::GLOBAL };
    let mut inventory_obj = InventoryObject::get(iid).unwrap();
    let tower = inventory_obj.object.get_the_tower_mut();
    unsafe {require(tower.lvl < 3)};
    let cost = UPGRADE_COST[tower.lvl as usize];
    unsafe {require(inventory_obj.reward >= cost)};
    inventory_obj.reward -= cost;
    inventory_obj.object.upgrade();
    inventory_obj.store();
}

impl State {
    pub fn run(&mut self) {
        self.counter += 1;

        let splen = self.spawners.len();
        let mlen = self.monsters.len();
        zkwasm_rust_sdk::dbg!("run monsters: {}\n", mlen);

        let mut collector = vec![];

        // figureout all the collectors in the state
        for obj in self.collectors.iter() {
            collector.push(obj.position.clone())
        }
        let mut termination_monster = vec![];
        let mut termination_drop = vec![];
        let mut spawn = vec![];
        let mut tower_range: Vec<(Tower<RectDirection>, RectCoordinate, usize, usize, usize)> =
            vec![];

        for (index, obj) in self.monsters.iter_mut().enumerate() {
            //let m = &obj.object;
            if collector.contains(&obj.position) {
                zkwasm_rust_sdk::dbg!("terminate: {}\n", index);
                termination_monster.push(index);
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

        for (index, obj) in self.towers.iter_mut().enumerate() {
            if let Object::Tower(tower) = &mut obj.object.object {
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

        for (index, obj) in self.monsters.iter_mut().enumerate() {
            for t in tower_range.iter_mut() {
                let range = t.0.range(&t.1, &obj.position);
                if range < t.2 {
                    t.2 = range;
                    t.4 = index;
                }
            }
        }

        let mut events = vec![];

        for t in tower_range.iter_mut() {
            if t.4 != usize::max_value() {
                let m = &mut self.monsters[t.4].object;
                let hit_reward = m.hit;
                if m.hp < t.0.power {
                    m.hp = 0;
                } else {
                    m.hp -= t.0.power;
                }
                if m.hp == 0 {
                    self.towers[t.3].object.reward += m.kill; // kill reward
                    termination_monster.push(t.4);
                    self.id_allocator += 1;
                    spawn.push(PositionedObject::new(
                        Object::Dropped(Dropped::new(10)),
                        self.monsters[t.4].position.clone(),
                        self.id_allocator,
                    ));
                }
                events.push(Event::Attack(
                    t.1.repr(),
                    self.monsters[t.4].position.repr(),
                    0,
                ));
                if let Object::Tower(tower) = &mut self.towers[t.3].object.object {
                    tower.count = tower.cooldown;
                }
                self.towers[t.3].object.reward += hit_reward; // hit reward
                self.towers[t.3].object.store();
            }
        }

        termination_monster.reverse();
        for idx in termination_monster {
            self.remove_monster(idx);
        }

        termination_drop.reverse();
        for idx in termination_drop {
            self.remove_monster(idx);
        }

        for obj in spawn.into_iter() {
            self.spawn(obj);
        }

        self.events = events;
    }
}
