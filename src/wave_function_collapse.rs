use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use nalgebra::{Vector2, vector};
use rand::random_range;

use crate::{
    app::{ALL_DIRS, MAP_HEIGHT, MAP_WIDTH, Rotate, Tile},
    direction::Direction,
};
pub static PROPAGATE_OFFSETS: LazyLock<HashMap<Direction, Vector2<i32>>> = LazyLock::new(|| {
    let mut offsets = HashMap::new();

    offsets.insert(Direction::Up, vector![0, -1]);
    offsets.insert(Direction::Down, vector![0, 1]);
    offsets.insert(Direction::Left, vector![-1, 0]);
    offsets.insert(Direction::Right, vector![1, 0]);

    offsets
});

pub type WfcCell = HashSet<Tile>;
pub trait WfcCellTrait {
    fn collapsed(&self) -> bool;
    fn entropy(&self) -> usize;
    fn single(&self) -> Tile;
    fn full() -> Self;
    fn collapse(&mut self);
    fn propagate(&mut self, dir: Direction, with: bool);
    fn any_contains(&self, dir: Direction) -> bool;
}
impl WfcCellTrait for WfcCell {
    fn collapsed(&self) -> bool {
        self.len() == 1
    }
    fn entropy(&self) -> usize {
        self.len()
    }
    fn single(&self) -> Tile {
        self.iter().nth(0).unwrap().clone()
    }
    fn full() -> Self {
        let mut cell = Self::with_capacity(ALL_DIRS.len());

        for dir in ALL_DIRS.clone() {
            cell.insert(dir);
        }

        cell
    }
    fn collapse(&mut self) {
        let rand = random_range(0..self.entropy());
        let tile = self
            .iter()
            .nth(rand)
            .expect(format!("Failed to collapse! rand: {}", rand).as_str())
            .clone();

        self.clear();
        self.insert(tile);
    }
    /// requires e.g. keep tiles that HAVE the dir
    fn propagate(&mut self, dir: Direction, requires: bool) {
        let mut to_remove = vec![];

        for tile in self.iter() {
            if tile.contains(&dir) != requires {
                to_remove.push(tile.clone());
            }
        }

        for tile in to_remove {
            self.remove(&tile);
        }
    }
    fn any_contains(&self, dir: Direction) -> bool {
        for set in self {
            if set.contains(&dir) {
                return true;
            }
        }

        false
    }
}
pub struct WaveFunctionCollapse {
    pub map: HashMap<Vector2<usize>, WfcCell>,
}
impl WaveFunctionCollapse {
    pub fn new() -> Self {
        let map = HashMap::with_capacity(MAP_WIDTH * MAP_HEIGHT);

        Self { map }
    }
    pub fn generate(&mut self) -> HashMap<Vector2<usize>, Tile> {
        self.init_map();
        self.correct_edges();

        while !self.all_collapsed() {
            self.wave();
        }

        self.convert()
    }
    fn wave(&mut self) {
        let pos = self.get_least_entropy_tile();

        let tile = if let Some(x) = self.map.get_mut(&pos) {
            x
        } else {
            return;
        };

        tile.collapse();
        self.propagate_neighbours(pos);
    }
    fn init_map(&mut self) {
        for y in 0..MAP_WIDTH {
            for x in 0..MAP_HEIGHT {
                let pos = vector![x, y];
                self.map.insert(pos, WfcCell::full());
            }
        }
    }
    fn correct_edges(&mut self) {
        for x in 0..MAP_WIDTH {
            let pos_1 = vector![x, 0];
            let pos_2 = vector![x, MAP_HEIGHT - 1];

            if let Some(tile) = self.map.get_mut(&pos_1) {
                tile.propagate(Direction::Up, false);
            }

            if let Some(tile) = self.map.get_mut(&pos_2) {
                tile.propagate(Direction::Down, false);
            }
        }

        for y in 0..MAP_HEIGHT {
            let pos_1 = vector![0, y];
            let pos_2 = vector![MAP_WIDTH - 1, y];

            if let Some(tile) = self.map.get_mut(&pos_1) {
                tile.propagate(Direction::Left, false);
            }

            if let Some(tile) = self.map.get_mut(&pos_2) {
                tile.propagate(Direction::Right, false);
            }
        }
    }
    fn convert(&mut self) -> HashMap<Vector2<usize>, Tile> {
        let mut new_map = HashMap::with_capacity(MAP_WIDTH * MAP_HEIGHT);

        for (pos, cell) in &self.map {
            let tile = cell.single();
            new_map.insert(*pos, tile);
        }

        self.shuffle(&mut new_map);

        new_map
    }
    fn all_collapsed(&self) -> bool {
        for tile in self.map.values() {
            if !tile.collapsed() {
                return false;
            }
        }

        true
    }
    fn get_least_entropy_tile(&self) -> Vector2<usize> {
        let mut best_entropy = usize::MAX;
        let mut best_pos = vector![random_range(0..MAP_WIDTH), random_range(0..MAP_HEIGHT)];

        for (pos, tile) in self.map.iter() {
            if tile.collapsed() {
                continue;
            }

            let entropy = tile.entropy();

            if entropy < best_entropy {
                best_entropy = entropy;
                best_pos = *pos;
            }
        }

        best_pos
    }
    fn propagate_neighbours(&mut self, pos: Vector2<usize>) {
        let tile_1 = if let Some(x) = self.map.get(&pos) {
            x.clone()
        } else {
            return;
        };

        for (dir, offset) in PROPAGATE_OFFSETS.clone() {
            let temp_pos = vector![pos.x as i32, pos.y as i32] + offset;

            if temp_pos.x >= 0
                && temp_pos.x < MAP_WIDTH as i32
                && temp_pos.y >= 0
                && temp_pos.y < MAP_HEIGHT as i32
            {
                let final_pos = vector![temp_pos.x as usize, temp_pos.y as usize];

                let tile_2 = if let Some(x) = self.map.get_mut(&final_pos) {
                    x
                } else {
                    return;
                };

                let opposite_dir = dir.opposite();

                tile_2.propagate(opposite_dir, tile_1.any_contains(dir));
            }
        }
    }
    fn shuffle(&mut self, tiles: &mut HashMap<Vector2<usize>, Tile>) {
        for tile in tiles.values_mut() {
            let rand = random_range(0..4);

            for _ in 0..rand {
                tile.rotate();
            }
        }
    }
}
