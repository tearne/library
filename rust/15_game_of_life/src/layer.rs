use std::collections::VecDeque;

use rgb::RGB8;

use crate::{dimensions::Dimensions, grid::Grid};

pub struct Layer {
    colour: RGB8,
    grids: VecDeque<Grid>,
}
impl Layer {
    pub fn new(colour: RGB8, dim: &Dimensions) -> Self {
        let mut grids = VecDeque::<Grid>::new();
        grids.push_front(Grid::random(dim));
        grids.push_front(Grid::random(dim));

        Layer {
            colour,
            grids,
        }
    }

    pub fn reset(&mut self, dim: &Dimensions) {
        self.grids = VecDeque::<Grid>::new();
        self.grids.push_front(Grid::random(dim));
        self.grids.push_front(Grid::random(dim));
    }

    pub fn interpolate_history(&self, px: usize, amount: f64) -> RGB8 {
        assert!((0.0..=1.0).contains(&amount));

        let previous_px = self.grids.get(1).unwrap().get(px);
        let current_px = self.current().get(px);

        let prev_value = if previous_px {1.0} else {0.0};
        let current_value = if current_px {1.0} else {0.0};
        let degree = prev_value * (1.0 - amount) + current_value * amount;

        RGB8::new(
            (self.colour.r as f64 * degree) as u8,
            (self.colour.g as f64 * degree) as u8,
            (self.colour.b as f64 * degree) as u8,
        )
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
        self.grids.front_mut().unwrap().set(idx, false);
    }
}
