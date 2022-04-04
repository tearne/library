use ndarray::{arr2, Array2, arr1, Array1};
use rgb::RGB8;

pub enum Rotation{
    NONE,
    QUARTER_CLOCK,
    HALF,
    QUARTER_COUNTER,
}

 pub struct Matrix{
    mat: Array2<i32>,
    offset: Array1<i32>,
 }
 impl Matrix {
     pub fn new(rotation: Rotation) -> Self {
         match rotation {
            Rotation::NONE => todo!(),
            Rotation::QUARTER_CLOCK => todo!(),
            Rotation::HALF => todo!(),
            Rotation::QUARTER_COUNTER => {
                Matrix{
                    mat: arr2(&[[0,1],[-1,0]]),
                    offset: arr1(&[0,15])
                }
            },
        }
     }

     pub fn apply(&self, px: &[RGB8]) -> Vec<RGB8> {
        assert!(px.len() == 256, "length is {}", px.len());
        
        let rejig_positions = px.iter()
            .enumerate()
            .map(|(idx, value)|{
                let x = (idx % 16) as i32;
                let y = (idx / 16) as i32;
                let rot = self.mat.dot( &arr1(&[x,y]) ) + &self.offset;
                println!(" --> {:#?},    {}  {}",rot, rot[0], rot[1]);
                let new_idx = (rot[0] + rot[1] * 16) as usize; 
                (new_idx, *value)
            });

        let mut rejig = [RGB8::new(0,0,0); 256];
        for (new_idx, value) in rejig_positions {
            rejig[new_idx] = value;
        }

        rejig.to_vec()
     }
 }