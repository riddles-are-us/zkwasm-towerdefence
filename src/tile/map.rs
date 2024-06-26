use super::coordinate::Coordinate;
use super::coordinate::Tile;
use crate::game::bigint_serializer;
use serde::Serialize;

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

#[derive(Clone, Serialize)]
pub struct Map<C: Coordinate> {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile<C, Option<C::Direction>>>,
}

impl<C: Coordinate> Map<C> {
    pub fn new(width: usize, height: usize, tiles: Vec<Tile<C, Option<C::Direction>>>) -> Self {
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
        self.tiles.get_mut(index).unwrap().set_feature(f)
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
