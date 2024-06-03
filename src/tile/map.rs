use super::coordinate::Coordinate;
use super::coordinate::Tile;
use crate::game::object::Object;
use crate::game::object::Monster;
use crate::game::object::Collector;
use crate::game::object::Spawner;
use crate::game::object::InventoryObject;
use crate::game::object::Dropped;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PositionedObject<C: Coordinate, Object: Clone> {
    pub position: C,
    pub object: Object,
}

impl<C: Coordinate, O: Clone> PositionedObject<C, O> {
    pub fn new(obj: O, pos: C) -> Self {
        PositionedObject {
            object: obj,
            position: pos,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Map<C: Coordinate> {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile<C, Option<C::Direction>>>,
    pub monsters: Vec<PositionedObject<C, Monster>>,
    pub drops: Vec<PositionedObject<C, Dropped>>,
    pub collectors: Vec<PositionedObject<C, Collector>>,
    pub spawners: Vec<PositionedObject<C, Spawner>>,
    pub towers: Vec<PositionedObject<C, InventoryObject>>,
}

impl<C: Coordinate> Map<C> {
    pub fn new(
        width: usize,
        height: usize,
        tiles: Vec<Tile<C, Option<C::Direction>>>,
    ) -> Self {
        Map {
            width,
            height,
            tiles,
            monsters: vec![],
            drops: vec![],
            towers: vec![],
            spawners: vec![],
            collectors: vec![],
        }
    }
    pub fn place_spawner_at(&mut self, object: Spawner, position: C) -> &PositionedObject<C, Spawner> {
        self.spawners.push(PositionedObject::new(object, position));
        self.spawners.get(self.spawners.len() - 1).unwrap()
    }

    pub fn place_collector_at(&mut self, object: Collector, position: C) -> &PositionedObject<C, Collector> {
        self.collectors.push(PositionedObject::new(object, position));
        self.collectors.get(self.collectors.len() - 1).unwrap()
    }

    pub fn place_tower_at(&mut self, object: InventoryObject, position: C) -> &PositionedObject<C, InventoryObject> {
        self.towers.push(PositionedObject::new(object, position));
        self.towers.get(self.towers.len() - 1).unwrap()
    }

    pub fn remove_tower(&mut self, index: usize) -> PositionedObject<C, InventoryObject> {
        self.towers.swap_remove(index)
    }

    pub fn spawn_monster_at(&mut self, object: Monster, position: C) -> &PositionedObject<C, Monster> {
        self.monsters.push(PositionedObject::new(object, position));
        self.monsters.get(self.monsters.len() - 1).unwrap()
    }

    pub fn remove_monster(&mut self, index: usize) -> PositionedObject<C, Monster> {
        self.monsters.swap_remove(index)
    }

    pub fn spawn_dropped_at(&mut self, object: Dropped, position: C) -> &PositionedObject<C, Monster> {
        self.drops.push(PositionedObject::new(object, position));
        self.monsters.get(self.monsters.len() - 1).unwrap()
    }

    pub fn remove_dropped(&mut self, index: usize) -> PositionedObject<C, Dropped> {
        self.drops.swap_remove(index)
    }

    pub fn spawn(&mut self, obj: PositionedObject<C, Object<C::Direction>>) {
        match obj.object {
            Object::Monster(m) => self.spawn_monster_at(m, obj.position),
            Object::Dropped(d) => self.spawn_dropped_at(d, obj.position),
            _ => unreachable!()
        };
    }


    pub fn coordinate_of_tile_index(&self, index: usize) -> C {
        C::new((index % self.width) as i64, (index / self.width) as i64)
    }

    pub fn index_of_tile_coordinate(&self, cor: &C) -> usize {
        let (x, y) = cor.repr();
        (x as usize) + (y as usize) * self.width
    }

    pub fn set_feature(&mut self, index: usize, f: Option<C::Direction>) {
        self.tiles.get_mut(index).unwrap().set_feature(f)
    }

    pub fn get_feature(&self, index: usize) -> Option<C::Direction> {
        self.tiles.get(index).unwrap().feature.clone()
    }

    /*
    pub fn get_neighbours<O: Clone>(
        &mut self,
        pos: &PositionedObject<C, O>,
        distance: u64,
        filter: impl Fn(&PositionedObject<C, O>) -> bool,
    ) -> Vec<&PositionedObject<C, O>> {
        let mut r = vec![];
        for obj in self.objects.iter() {
            if C::distance(&obj.position, &pos.position) < distance {
                if filter(obj) {
                    r.push(obj)
                }
            }
        }
        r
    }
    */
}
