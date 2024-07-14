use super::event::Event;
use super::object::Collector;
use super::object::Dropped;
use super::object::Monster;
use super::object::Object;
use super::object::Spawner;
use super::object::Tower;
use crate::player::Player;
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
use zkwasm_rust_sdk::wasm_dbg;
//use zkwasm_rust_sdk::require;

// The global state
#[derive(Clone, Serialize)]
pub struct State {
    #[serde(skip_serializing)]
    pub id_allocator: u64,
    pub map: Map<RectCoordinate>,
    pub monsters: Vec<PositionedObject<RectCoordinate, Monster>>,
    pub drops: Vec<PositionedObject<RectCoordinate, Dropped>>,
    pub collectors: Vec<PositionedObject<RectCoordinate, Collector>>,
    pub spawners: Vec<PositionedObject<RectCoordinate, Spawner>>,
    pub towers: Vec<PositionedObject<RectCoordinate, InventoryObject>>,
    pub events: Vec<Event>,
}

impl State {
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
    ) -> &PositionedObject<RectCoordinate, InventoryObject> {
        unsafe {
            require(self.map.get_occupy(&position) == 0);
        }
        self.id_allocator += 1;
        self.map.set_occupy(&position, 1);
        self.towers
            .push(PositionedObject::new(object, position, self.id_allocator));
        self.towers.get(self.towers.len() - 1).unwrap()
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

pub fn handle_place_tower(iid: &[u64; 4], pos: usize) {
    let global = unsafe { &mut crate::config::GLOBAL };
    let inventory_obj = InventoryObject::get(iid);
    let position = global.map.coordinate_of_tile_index(pos);
    global.place_tower_at(inventory_obj.unwrap(), position);
}

pub fn handle_update_inventory(iid: &[u64; 4], feature: u64, pid: &[u64; 4]) {
    let mut inventory_obj = InventoryObject::get(iid);
    if let Some(inventory_obj) = inventory_obj.as_mut() {
        let tower = inventory_obj.object.get_the_tower_mut();
        tower.owner[0] = pid[1];
        tower.owner[1] = pid[2];
        inventory_obj.store();
    } else {
        let mut tower = CONFIG.standard_towers[feature as usize].clone();
        tower.owner[0] = pid[1];
        tower.owner[1] = pid[2];
        let inventory_obj = InventoryObject::new(iid.clone(), Object::Tower(tower), 10);
        inventory_obj.store();
    }
}

pub fn handle_claim_tower(nonce: u64, iid: &[u64; 4], pid: &[u64; 4]) {
    let inventory_obj = InventoryObject::get(iid);
    if inventory_obj.is_none() {
        unreachable!()
    } else {
        let obj = inventory_obj.unwrap().object.clone();
        let tower = obj.get_the_tower();
        unsafe {
            zkwasm_rust_sdk::require(tower.owner[0] == pid[1]);
            zkwasm_rust_sdk::require(tower.owner[1] == pid[2]);
        }

        let mut player_opt = Player::get(pid);
        if let Some(player) = player_opt.as_mut() {
            player.check_and_inc_nonce(nonce);
            if !player.owns(iid[0]) {
                player.inventory.push(iid[0]);
                player.store()
            }
        } else {
            unsafe { zkwasm_rust_sdk::require(nonce == 0) };
            let player = Player {
                nonce,
                player_id: *pid,
                inventory: vec![iid[0]],
            };
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
        //let global = unsafe { &mut crate::config::GLOBAL };
        //let map = unsafe { &crate::config::GLOBAL.map };
        //let monsters = &mut self.map.monsters;
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
