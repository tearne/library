use std::collections::VecDeque;

use rgb::RGB8;

use crate::{dimensions::Dimensions, grid::Grid};

pub struct Layer {
    pub colour: RGB8,
    grids: VecDeque<Grid>,
}
impl Layer {
    pub fn new(colour: RGB8, dim: &Dimensions) -> Self {
        let mut grids = VecDeque::<Grid>::new();
        grids.push_front(Grid::random(dim));

        Layer {
            colour,
            grids,
        }
    }

    pub fn reset(&mut self, dim: &Dimensions) {
        self.grids = VecDeque::<Grid>::new();
        self.grids.push_front(Grid::random(dim));
    }

    pub fn current(&self) -> &Grid {
        self.grids.front().unwrap()
    }

    pub fn has_repition(&self) -> bool {
        let current = self.grids.front().unwrap();
        self.grids.iter().skip(1).any(|x| x == current)
    }

    pub fn evolve(&mut self, dim: &Dimensions) {
        let next = self.grids.front().unwrap().evolve(dim);
        self.grids.push_front(next);
        self.grids.truncate(3);
    }

    pub fn purge(&mut self, idx: usize) {
        self.grids.front_mut().unwrap().px[idx] = false;
    }
}
