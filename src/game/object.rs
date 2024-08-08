use crate::game::serialize::U64arraySerialize;
use serde::Serialize;
use std::slice::IterMut;

use crate::MERKLE_MAP;
use crate::{
    config::upgrade_tower,
    tile::coordinate::{Coordinate, RectCoordinate, RectDirection},
};


pub fn to_full_obj_id(id: u64) -> [u64; 4] {
    [id, 0xffff, 0xff01, 0xff02]
}

#[derive(Clone, Serialize)]
pub struct Monster {
    pub born: u64,
    pub hp: u64,
    pub hit: u64,
    pub kill: u64,
}

impl Monster {
    pub fn new(hp: u64, hit: u64, kill: u64) -> Self {
        Monster {born: hp, hp, hit, kill}
    }
}

impl U64arraySerialize for Monster {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        data.append(&mut vec![self.born, self.hp, self.hit, self.kill])
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Monster {
            born: *(data.next().unwrap()),
            hp: *(data.next().unwrap()),
            hit: *data.next().unwrap(),
            kill: *data.next().unwrap(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Tower<Direction: Clone + Serialize> {
    pub lvl: u64,
    pub range: u64,
    pub power: u64,
    pub cooldown: u64,
    pub count: u64,
    #[serde(skip_serializing)]
    pub owner: [u64; 2], // tail of the pubkey of the owner
    pub direction: Direction,
}

impl Tower<RectDirection> {
    pub fn new(
        lvl: u64,
        range: u64,
        power: u64,
        cooldown: u64,
        owner: [u64; 2],
        direction: RectDirection,
    ) -> Self {
        Tower {
            lvl,
            range,
            power,
            cooldown,
            count: cooldown, // initial count
            owner,
            direction,
        }
    }
    pub fn range(&self, src: &RectCoordinate, target: &RectCoordinate) -> usize {
        let src = src.repr();
        let target = target.repr();
        match self.direction {
            RectDirection::Left => {
                if src.1 == target.1 {
                    if target.0 < src.0 {
                        (src.0 - target.0) as usize
                    } else {
                        usize::max_value()
                    }
                } else {
                    usize::max_value()
                }
            }
            RectDirection::Right => {
                if src.1 == target.1 {
                    if target.0 > src.0 {
                        (target.0 - src.0) as usize
                    } else {
                        usize::max_value()
                    }
                } else {
                    usize::max_value()
                }
            }
            RectDirection::Top => {
                if src.0 == target.0 {
                    if target.1 < src.1 {
                        (src.1 - target.1) as usize
                    } else {
                        usize::max_value()
                    }
                } else {
                    usize::max_value()
                }
            }
            RectDirection::Bottom => {
                if src.0 == target.0 {
                    if target.1 > src.1 {
                        (target.1 - src.1) as usize
                    } else {
                        usize::max_value()
                    }
                } else {
                    usize::max_value()
                }
            }
        }
    }
}

impl U64arraySerialize for Tower<RectDirection> {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        data.append(&mut vec![
            self.lvl,
            self.range,
            self.power,
            self.cooldown,
            self.owner[0],
            self.owner[1],
            self.direction.clone() as u64,
        ]);
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        let directions = RectCoordinate::directions();
        Self::new(
            *(data.next().unwrap()),
            *data.next().unwrap(),
            *data.next().unwrap(),
            *data.next().unwrap(),
            [*data.next().unwrap(), *data.next().unwrap()],
            directions[*data.next().unwrap() as usize].clone(),
        )
    }
}

#[derive(Clone, Serialize)]
pub struct Spawner {
    pub rate: u64,
    pub count: u64,
}

impl Spawner {
    pub fn new(rate: u64, count: u64) -> Self {
        Spawner { rate, count }
    }
}

impl U64arraySerialize for Spawner {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        data.append(&mut vec![self.rate, self.count]);
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Self::new(*(data.next().unwrap()), *data.next().unwrap())
    }
}

#[derive(Clone, Serialize)]
pub struct Collector {
    buf: u64,
}

impl Collector {
    pub fn new(buf: u64) -> Self {
        Collector { buf }
    }
}

impl U64arraySerialize for Collector {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        data.push(self.buf);
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Self::new(*(data.next().unwrap()))
    }
}

#[derive(Clone, Serialize)]
pub struct Dropped {
    pub delta: u64,
}

impl Dropped {
    pub fn new(delta: u64) -> Self {
        Dropped { delta }
    }
}

impl U64arraySerialize for Dropped {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        data.push(self.delta);
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Self::new(*(data.next().unwrap()))
    }
}

#[derive(Clone, Serialize)]
pub enum Object<Direction: Clone + Serialize> {
    Monster(Monster),
    Tower(Tower<Direction>),
    Spawner(Spawner),
    Dropped(Dropped),
    Collector(Collector),
}

impl U64arraySerialize for Object<RectDirection> {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        match self {
            Object::Monster(o) => {
                data.push(0);
                o.to_u64_array(data);
            },
            Object::Tower(o) => {
                data.push(1);
                o.to_u64_array(data);
            },
            Object::Spawner(o) => {
                data.push(2);
                o.to_u64_array(data);
            }
            Object::Dropped(o) => {
                data.push(3);
                o.to_u64_array(data);
            },
            Object::Collector(o) => {
                data.push(4);
                o.to_u64_array(data);
            }
        };
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        let t: u64 = *(data.next().unwrap());
        match t {
            0 => Object::Monster(Monster::from_u64_array(data)),
            1 => Object::Tower(Tower::from_u64_array(data)),
            2 => Object::Spawner(Spawner::from_u64_array(data)),
            3 => Object::Dropped(Dropped::from_u64_array(data)),
            4 => Object::Collector(Collector::from_u64_array(data)),
            _ => unreachable!(),
        }
    }
}

impl Object<RectDirection> {
    pub fn get_the_tower(&self) -> &Tower<RectDirection> {
        match self {
            Object::Tower(t) => t,
            _ => {
                todo!()
            }
        }
    }
    pub fn get_the_tower_mut(&mut self) -> &mut Tower<RectDirection> {
        match self {
            Object::Tower(t) => t,
            _ => {
                todo!()
            }
        }
    }

    pub fn upgrade(&mut self) {
        match self {
            Object::Tower(t) => upgrade_tower(t),
            _ => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Serialize)]
pub struct InventoryObject {
    pub object_id: [u64; 4],
    pub object: Object<RectDirection>,
    pub reward: u64,
}

impl InventoryObject {
    pub fn new(object_id: [u64; 4], object: Object<RectDirection>) -> Self {
        Self {
            object_id,
            object,
            reward: 0,
        }
    }
}

impl U64arraySerialize for InventoryObject {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        self.object.to_u64_array(data);
        data.push(self.reward);
        data.push(self.object_id[0]);
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        let object = Object::from_u64_array(data);
        let reward = *(data.next().unwrap());
        let oid = *(data.next().unwrap());
        InventoryObject {
           object_id: to_full_obj_id(oid),
           reward,
           object,
        }
    }
}

impl InventoryObject {
    pub fn get(object_id: &[u64; 4]) -> Option<Self> {
        let kvpair = unsafe { &mut MERKLE_MAP };
        zkwasm_rust_sdk::dbg!("get object with oid {:?}\n", object_id);
        let mut data = kvpair.get(&object_id);
        zkwasm_rust_sdk::dbg!("get object with {:?}\n", data);
        if data.is_empty() {
            None
        } else {
            let mut slice_iter = data.as_mut_slice().iter_mut();
            let o = Object::from_u64_array(&mut slice_iter);
            let inventory_obj = InventoryObject {
                object_id: object_id.clone(),
                reward: *(slice_iter.next().unwrap()),
                object: o,
            };
            Some(inventory_obj)
        }
    }
    pub fn store(&self) {
        let oid = self.object_id;
        let mut data = Vec::with_capacity(1024);
        self.object.to_u64_array(&mut data);
        data.push(self.reward);
        let kvpair = unsafe { &mut MERKLE_MAP };
        kvpair.set(&self.object_id, data.as_slice());
    }
}
