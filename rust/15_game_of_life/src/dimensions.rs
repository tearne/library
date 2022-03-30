#[derive(Debug)]
pub struct Dimensions {
    pub width: u8,
    pub height: u8,
    pub offsets: Vec<(u8,u8)>,
}
impl Dimensions {
    pub fn new(width: u8, height: u8) -> Self {
        let offsets = {
            vec![
                (-1i16,-1i16), (0,-1), (1,-1), (-1,0), 
                (1,0), (-1,1), (0,1), (1,1)
            ]
                .iter()
                //Add screen width or height to everything to get rid of the negatives
                .map( |(x,y)| ((x+width as i16) as u8, (y+height as i16) as u8) )
                .collect::<Vec<(u8,u8)>>()
        };
        
        Dimensions { 
            width, 
            height, 
            offsets
        }
    }

    pub fn num_pixels(&self) -> u8 {
        self.width * self.height
    }

    pub fn to_idx(&self, xy: (u8,u8)) -> usize {
        xy.0 as usize + xy.1 as usize * self.width as usize
    }
    
    pub fn to_xy(&self, idx: usize) -> (u8, u8) {
        let x = (idx % self.width as usize) as u8;
        let y = (idx / self.width as usize) as u8; 
        (x,y)
    }
}



#[cfg(test)]
mod tests {
    use crate::dimensions::Dimensions;

    #[test]
    fn coords_to_index() {
        let dims = Dimensions::new(16, 16);
        assert_eq!(5, dims.to_idx((5,0)));
        assert_eq!(16, dims.to_idx((0,1)));
        assert_eq!(255, dims.to_idx((15,15)));

        let dims = Dimensions::new(8, 4);
        assert_eq!(5, dims.to_idx((5,0)));
        assert_eq!(15, dims.to_idx((7,1)));
        assert_eq!(19, dims.to_idx((3,2)));
    }

    #[test]
    fn index_index() {
        let dims = Dimensions::new(16, 16);
        assert_eq!((0,0), dims.to_xy(0));
        assert_eq!((0,1), dims.to_xy(16));
        assert_eq!((0,2), dims.to_xy(32));
        assert_eq!((0,14), dims.to_xy(224));
        assert_eq!((5,14), dims.to_xy(229));

        let dims = Dimensions::new(17, 7);
        assert_eq!((0,0), dims.to_xy(0));
        assert_eq!((16,0), dims.to_xy(16));
        assert_eq!((0,1), dims.to_xy(17));
        assert_eq!((16,1), dims.to_xy(33));
        assert_eq!((16,6), dims.to_xy(118));
    }

    #[test]
    fn round_trip() {
        let dims = Dimensions::new(16, 16);

        assert_eq!(0, dims.to_idx(dims.to_xy(0)) );
        assert_eq!(20, dims.to_idx(dims.to_xy(20)) );
        assert_eq!(60, dims.to_idx(dims.to_xy(60)) );
        assert_eq!(180, dims.to_idx(dims.to_xy(180)) );
        assert_eq!(255, dims.to_idx(dims.to_xy(255)) );
        
        assert_eq!((3,5), dims.to_xy(dims.to_idx((3,5))) );
        assert_eq!((13,14), dims.to_xy(dims.to_idx((13,14))) );
        assert_eq!((7,5), dims.to_xy(dims.to_idx((7,5))) );
        assert_eq!((0,5), dims.to_xy(dims.to_idx((0,5))) );
        assert_eq!((15,15), dims.to_xy(dims.to_idx((15,15))) );


        let dims = Dimensions::new(17, 7);

        assert_eq!(0, dims.to_idx(dims.to_xy(0)) );
        assert_eq!(20, dims.to_idx(dims.to_xy(20)) );
        assert_eq!(60, dims.to_idx(dims.to_xy(60)) );
        assert_eq!(118, dims.to_idx(dims.to_xy(118)) );
        
        assert_eq!((3,5), dims.to_xy(dims.to_idx((3,5))) );
        assert_eq!((13,2), dims.to_xy(dims.to_idx((13,2))) );
        assert_eq!((7,5), dims.to_xy(dims.to_idx((7,5))) );
        assert_eq!((0,5), dims.to_xy(dims.to_idx((0,5))) );
        assert_eq!((15,0), dims.to_xy(dims.to_idx((15,0))) );
    }
}