use crate::dimensions::Dimensions;

use rand::Rng;

#[derive(Default, PartialEq)]
pub struct Grid(Vec<bool>);
impl Grid {
    pub fn random(dim: &Dimensions) -> Self {
        let mut rng = rand::thread_rng();
        let px: Vec<_> = (0..dim.width * dim.height).map(|_|{ rng.gen_range(0..8) < 1 }).collect();
        Grid(px)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn set(&mut self, px: usize, value: bool) {
        self.0[px] = value;
    }

    pub fn get(&self, px: usize) -> bool {
        self.0[px]
    }

    pub fn iter(&self) -> GridIter {
        GridIter{ inner: self.0.iter() }
    }

    pub fn num_live_neighbours_at(&self, idx: usize, dim: &Dimensions) -> u8 {
        let xy = dim.to_xy(idx);
        
        let is_alive_at = |xy:(u8,u8)| -> bool {
            self.0[dim.to_idx(xy)]
        };

        let count = dim.offsets.iter().map(|(a,b)|{
            let nbr = ((xy.0 + a)%dim.width, (xy.1 + b)%dim.height);
            is_alive_at(nbr)
        }).fold(0,|acc,next_is_alive|{
            if !next_is_alive {acc}
            else {acc+1}
        }) as u8;

        count
    }

    pub fn evolve(&self, dim: &Dimensions) -> Self {
        let new_px: Vec<bool> = (0..self.0.len())
            .map(|i: usize|{
                let is_alive = self.0[i];
                let nbr_count = self.num_live_neighbours_at(i, dim);
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
        
        Grid(new_px)
    }
}

pub struct GridIter<'a> {
    inner: core::slice::Iter<'a, bool>,
}
impl<'a> Iterator for GridIter<'a> {
    type Item = &'a bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}