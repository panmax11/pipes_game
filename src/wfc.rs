use std::collections::HashSet;

use crate::app::Tile;

pub struct WfcCell {
    pub possible_tiles: HashSet<Tile>,
    pub collapsed: bool,
}
impl WfcCell {
    pub fn new(possible_tiles: HashSet<Tile>) -> Self {
        Self {
            possible_tiles,
            collapsed: false,
        }
    }
}
