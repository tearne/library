use std::collections::VecDeque;

use rgb::RGB8;

use once_cell::sync::Lazy;
use rand::Rng;

use crate::coords;

pub struct Layer {
    pub colour: RGB8,
    grids: VecDeque<Grid>,
}
impl Layer {
    pub fn new(colour: RGB8) -> Self {
        let mut grids = VecDeque::<Grid>::new();
        grids.push_front(Grid::random());

        Layer {
            colour,
            grids,
        }
    }

    pub fn reset(&mut self) {
        self.grids = VecDeque::<Grid>::new();
        self.grids.push_front(Grid::random());
    }

    pub fn current(&self) -> &Grid {
        self.grids.front().unwrap()
    }

    pub fn has_repition(&self) -> bool {
        let current = self.grids.front().unwrap();
        self.grids.iter().skip(1).any(|x| x == current)
    }

    pub fn evolve(&mut self) {
        let next = self.grids.front().unwrap().evolve();
        self.grids.push_front(next);
        self.grids.truncate(3);
    }

    pub fn purge(&mut self, idx: usize) {
        self.grids.front_mut().unwrap().0[idx] = false;
    }
}

static OFFSETS: Lazy<Vec<(u8, u8)>> = Lazy::new(|| {
    vec![
        (-1,-1),
        (0,-1),
        (1,-1),
        (-1,0),
        (1,0),
        (-1,1),
        (0,1),
        (1,1)
    ].iter().map(|(x,y)|((x+16) as u8, (y+16) as u8))
    .collect::<Vec<(u8,u8)>>()
});

#[derive(Default, PartialEq)]
pub struct Grid(pub Vec<bool>);
impl Grid {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let vec: Vec<_> = (0..256).map(|_|{ rng.gen_range(0..10) < 2 }).collect();
        Grid(vec)
    }

    pub fn num_live_neighbours_at(&self, xy:(u8,u8)) -> u8 {
        let is_alive_at = |xy:(u8,u8)| -> bool {
            self.0[coords::to_idx(xy)] 
        };

        let count = OFFSETS.iter().map(|(a,b)|{
            let offset_xy = ((xy.0 + a)%16, (xy.1 + b)%16);
            is_alive_at(offset_xy)
        }).fold(0,|acc,next_is_alive|{
            if !next_is_alive {acc}
            else {acc+1}
        }) as u8;

        count
    }

    pub fn evolve(&self) -> Self {
        let new_grid: Vec<bool> = (0..self.0.len())
            .map(|i: usize|{
                let is_alive = self.0[i];
                let nbr_count = self.num_live_neighbours_at(coords::to_xy(i));
                if !is_alive {
                    // Dead cell comes to life if 3 live neighbours
                    nbr_count == 3 
                } 
                else { 
                    // Live cell dies if < 2 or > 3 live neighbours
                    (2..=3).contains(&nbr_count) 
                }
            })
            .collect();
        
        Grid(new_grid)
    }
}