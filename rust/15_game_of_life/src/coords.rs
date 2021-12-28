pub fn to_idx(xy: (u8,u8)) -> usize {
    (xy.0 + xy.1 * 16) as usize
}

pub fn to_xy(idx: usize) -> (u8, u8) {
    let x = (idx % 16) as u8;
    let y = (idx / 16) as u8; 
    (x,y)
}

#[cfg(test)]
mod tests {
    use crate::coords::{to_idx, to_xy};

    #[test]
    fn coords_to_index() {
        assert_eq!(5, to_idx((5,0)));
        assert_eq!(16, to_idx((0,1)));
        assert_eq!(255, to_idx((15,15)));
    }

    #[test]
    fn index_index() {
        assert_eq!((0,0), to_xy(0));
        assert_eq!((0,1), to_xy(16));
        assert_eq!((0,2), to_xy(32));
        assert_eq!((0,14), to_xy(224));
        assert_eq!((5,14), to_xy(229));
    }

    #[test]
    fn round_trip() {
        assert_eq!(0, to_idx(to_xy(0)) );
        assert_eq!(20, to_idx(to_xy(20)) );
        assert_eq!(60, to_idx(to_xy(60)) );
        assert_eq!(180, to_idx(to_xy(180)) );
        assert_eq!(255, to_idx(to_xy(255)) );
        
        assert_eq!((3,5), to_xy(to_idx((3,5))) );
        assert_eq!((13,14), to_xy(to_idx((13,14))) );
        assert_eq!((7,5), to_xy(to_idx((7,5))) );
        assert_eq!((0,5), to_xy(to_idx((0,5))) );
        assert_eq!((15,15), to_xy(to_idx((15,15))) );
    }
}