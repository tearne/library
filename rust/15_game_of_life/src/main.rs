pub mod coords;
pub mod layer;

use std::time::Duration;

use layer::Layer;
use rgb::RGB8;
use unicorn::pimoroni::unicorn::Unicorn;

static BLACK: RGB8 = RGB8::new(0,0,0);
static GREEN: RGB8 = RGB8::new(0, 20, 0);
static BLUE: RGB8 = RGB8::new(0, 0, 40);
static RED: RGB8 = RGB8::new(40,0,0);

fn intersect(layers: &[Layer]) -> Vec<usize> {
    (0usize..256)
        .filter_map(|idx| {
            let num_alive = layers.iter().fold(0,|acc,layer|{
                if layer.current().0[idx] { acc + 1 }
                else { acc }
            });
            if num_alive > 0 { Some(idx) }
            else { None }
        })
        .collect()
}

fn main() {
    let mut display = Unicorn::new();
    
    let mut layers = vec![
        Layer::new(GREEN), 
        Layer::new(RED), 
        // Layer::new(BLUE)
    ];

    loop {
        let mut pxls = vec![BLACK; 256];

        for layer in layers.iter_mut() {
            for (id, &alive) in layer.current().0.iter().enumerate() {
                if alive { pxls[id] += layer.colour; }
            }
            
            if layer.has_repition() { layer.reset(); }
            else { layer.evolve() }
        }

        for (idx, colour) in pxls.iter().enumerate() {
            display.set_idx(idx, colour )
        }

        display.flush();

        // for idx in intersect(&layers) {
        //     println!("-{}", &idx);
        //     for layer in layers.iter_mut() {
        //         layer.purge(idx)
        //     }
        // }

        std::thread::sleep(Duration::from_millis(500));
    }
}
