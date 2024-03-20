use serde::Serialize;

use crate::tile::coordinate::{Coordinate, RectCoordinate, RectDirection};

#[derive (Clone, Serialize)]
pub struct Monster {
    pub hp: u64,
    pub range: u64,
    pub power: u64,
}

impl Monster {
    pub fn new(hp: u64, range: u64, power: u64) -> Self {
        Monster {
            hp,
            range,
            power
        }
    }
}

#[derive (Clone, Serialize)]
pub struct Tower<Direction: Clone + Serialize>{
    range: u64,
    power: u64,
    pub cooldown: u64,
    pub count: u64,
    direction: Direction,
}

impl Tower<RectDirection> {
    pub fn new(range: u64, power: u64, cooldown: u64, count: u64,  direction: RectDirection) -> Self {
        Tower {
            range,
            power,
            cooldown,
            count,
            direction
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
            },
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
            },
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
            },
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

            },

        }
    }
}

#[derive (Clone, Serialize)]
pub struct Spawner{
    pub rate: u64,
    pub count: u64
}

impl Spawner {
    pub fn new(rate: u64, count: u64) -> Self {
        Spawner {
            rate,
            count
        }
    }
}

#[derive (Clone, Serialize)]
pub struct Collector{
    buf: u64,
}

impl Collector {
    pub fn new(buf: u64) -> Self {
        Collector {
           buf
        }
    }
}

#[derive (Clone, Serialize)]
pub struct Dropped {
    pub delta: u64,
}

impl Dropped {
    pub fn new(delta: u64) -> Self {
        Dropped {
           delta
        }
    }
}



#[derive (Clone, Serialize)]
pub enum Object<Direction: Clone + Serialize> {
    Monster(Monster),
    Tower(Tower<Direction>),
    Spawner(Spawner),
    Dropped(Dropped),
    Collector(Collector),
}





