use serde::Serialize;
use std::slice::IterMut;
use crate::config::UPGRADE_COST_MODIFIER;

use crate::{
    config::upgrade_tower,
    tile::coordinate::{Coordinate, RectCoordinate, RectDirection},
};
use crate::MERKLE_MAP;

pub trait U64arraySerialize {
    fn to_u64_array(&self) -> Vec<u64>;
    fn from_u64_array(data: &mut IterMut<u64>) -> Self;
}

#[derive(Clone, Serialize)]
pub struct Monster {
    pub hp: u64,
    pub range: u64,
    pub power: u64,
}

impl Monster {
    pub fn new(hp: u64, range: u64, power: u64) -> Self {
        Monster { hp, range, power }
    }
}

impl U64arraySerialize for Monster {
    fn to_u64_array(&self) -> Vec<u64> {
        vec![self.hp, self.range, self.power]
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Monster {
            hp: *(data.next().unwrap()),
            range: *data.next().unwrap(),
            power: *data.next().unwrap(),
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
    direction: Direction,
}

impl Tower<RectDirection> {
    pub fn new(lvl: u64, range: u64, power: u64, cooldown: u64, direction: RectDirection) -> Self {
        Tower {
            lvl,
            range,
            power,
            cooldown,
            count: cooldown, // initial count
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
    fn to_u64_array(&self) -> Vec<u64> {
        vec![self.lvl, self.range, self.power, self.cooldown, self.direction.clone() as u64]
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        let directions = RectCoordinate::directions();
        Self::new(
            *(data.next().unwrap()),
            *data.next().unwrap(),
            *data.next().unwrap(),
            *data.next().unwrap(),
            directions[*data.next().unwrap() as usize].clone()
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
    fn to_u64_array(&self) -> Vec<u64> {
        vec![self.rate, self.count]
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Self::new(
            *(data.next().unwrap()),
            *data.next().unwrap(),
        )
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
    fn to_u64_array(&self) -> Vec<u64> {
        vec![self.buf]
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Self::new(
            *(data.next().unwrap()),
        )
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
    fn to_u64_array(&self) -> Vec<u64> {
        vec![self.delta]
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        Self::new(
            *(data.next().unwrap()),
        )
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
    fn to_u64_array(&self) -> Vec<u64> {
        let (mut data, t) = match self {
            Object::Monster(o) => (o.to_u64_array(), 0),
            Object::Tower(o) => (o.to_u64_array(), 1),
            Object::Spawner(o) => (o.to_u64_array(), 2),
            Object::Dropped(o) => (o.to_u64_array(), 3),
            Object::Collector(o) => (o.to_u64_array(), 4),
        };
        data.insert(0,t);
        data
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        let t:u64 = *(data.next().unwrap());
        match t {
            0 => Object::Monster(Monster::from_u64_array(data)),
            1 => Object::Monster(Monster::from_u64_array(data)),
            2 => Object::Monster(Monster::from_u64_array(data)),
            3 => Object::Monster(Monster::from_u64_array(data)),
            4 => Object::Monster(Monster::from_u64_array(data)),
            _ => unreachable!()
        }
    }
}

impl Object<RectDirection> {
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
    pub cost: u64,
    pub upgrade_modifier: u64,
    pub reward: u64,
}

impl InventoryObject {
    pub fn new(object_id: [u64; 4], object: Object<RectDirection>, cost: u64) -> Self {
        Self {
            object_id,
            cost,
            upgrade_modifier: UPGRADE_COST_MODIFIER,
            object,
            reward: 0,
        }
    }
}

impl InventoryObject {
    pub fn get(object_id: &[u64; 4]) -> Option<Self> {
        let kvpair = unsafe {&mut MERKLE_MAP};
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
                cost: *(slice_iter.next().unwrap()),
                upgrade_modifier: *(slice_iter.next().unwrap()),
                reward: *(slice_iter.next().unwrap()),
                object: o,
            };
            Some(inventory_obj)
        }
    }
    pub fn store(&self) {
        let oid = self.object_id;
        zkwasm_rust_sdk::dbg!("store object {:?}\n", oid);
        let mut data = self.object.to_u64_array();
        data.push(self.cost);
        data.push(self.upgrade_modifier);
        data.push(self.reward);
        let kvpair = unsafe {&mut MERKLE_MAP};
        kvpair.set(&self.object_id, data.as_slice());
        zkwasm_rust_sdk::dbg!("end store object\n");
        todo!()
    }
}


