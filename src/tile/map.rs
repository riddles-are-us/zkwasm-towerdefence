use super::coordinate::Coordinate;
use super::coordinate::Tile;
use crate::game::bigint_serializer;
use crate::game::object::Object;
use serde::Serialize;
use crate::game::serialize::U64arraySerialize;
use core::slice::IterMut;

#[derive(Clone, Serialize)]
pub struct PositionedObject<C: Coordinate, Object: Clone> {
    #[serde(serialize_with = "bigint_serializer")]
    pub id: u64, // unique id in the map
    pub position: C,
    pub object: Object,
}

impl<C: Coordinate, O: Clone> PositionedObject<C, O> {
    pub fn new(obj: O, pos: C, id: u64) -> Self {
        PositionedObject {
            id,
            object: obj,
            position: pos,
        }
    }
}

fn cor_to_u64<C: Coordinate>(c: &C) -> u64 {
    let (x, y) = c.repr();
    ((x as u64) << 32) + ((y as u32) as u64)
}

fn u64_to_cor<C: Coordinate>(u: u64) -> C {
    C::new((u >> 32) as i64, (u & 0xffffffff) as i64)
}


impl<C: Coordinate, O: Clone + U64arraySerialize> U64arraySerialize for PositionedObject<C, O> {
    fn to_u64_array(&self, data: &mut Vec<u64>) {
        let index = cor_to_u64(&self.position);
        data.push(self.id);
        data.push(index);
        self.object.to_u64_array(data);
    }
    fn from_u64_array(data: &mut IterMut<u64>) -> Self {
        let id = *(data.next().unwrap());
        let position = u64_to_cor(*(data.next().unwrap()));
        let object = O::from_u64_array(data);
        PositionedObject {
            id,
            position,
            object,
        }
    }
    fn modify_from_u64_array(&mut self, data:&mut IterMut<u64>) {
        self.id = *(data.next().unwrap());
        self.position = u64_to_cor(*(data.next().unwrap()));
        self.object = O::from_u64_array(data);
    }
}


#[derive(Clone, Serialize)]
pub struct Map<C: Coordinate> {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile<Option<C::Direction>>>,
}

impl<C: Coordinate> Map<C> {
    pub fn new(width: usize, height: usize, tiles: Vec<Tile<Option<C::Direction>>>) -> Self {
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn coordinate_of_tile_index(&self, index: usize) -> C {
        C::new((index % self.width) as i64, (index / self.width) as i64)
    }

    pub fn index_of_tile_coordinate(&self, cor: &C) -> usize {
        let (x, y) = cor.repr();
        (x as usize) + (y as usize) * self.width
    }

    pub fn set_feature(&mut self, index: usize, f: Option<C::Direction>) {
        let tile = self.tiles.get_mut(index).unwrap();
        tile.set_feature(f);
        tile.occupied = 1;
    }

    pub fn get_feature(&self, index: usize) -> Option<C::Direction> {
        self.tiles.get(index).unwrap().feature.clone()
    }

    pub fn set_occupy(&mut self, cor: &C, indicator: u32) {
        let index = self.index_of_tile_coordinate(cor);
        self.tiles.get_mut(index).unwrap().occupied = indicator;
    }

    pub fn get_occupy(&mut self, cor: &C) -> u32 {
        let index = self.index_of_tile_coordinate(cor);
        self.tiles.get_mut(index).unwrap().occupied
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
