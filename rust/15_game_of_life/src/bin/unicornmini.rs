use std::time::Duration;

use gol::dimensions::Dimensions;
use gol::grid::Grid;
use gol::layer::Layer;
use rgb::RGB8;
use tokio::runtime::Runtime;
use unicorn::pimoroni::unicornmini::UnicornMini;

static BLACK: RGB8 = RGB8::new(0,0,0);
static GREEN: RGB8 = RGB8::new(0, 15, 0);
static BLUE: RGB8 = RGB8::new(0, 0, 40);
static RED: RGB8 = RGB8::new(40,0,0);

fn intersect(layers: &[Layer], dim: &Dimensions) -> Vec<usize> {
    (0usize..dim.num_pixels() as usize)
        .filter(|&idx| {
            let num_alive = layers.iter().fold(0,|acc,layer|{
                if layer.current().get(idx) { acc + 1 }
                else { acc }
            });
            num_alive > 1
        })
        .collect()
}

fn main() {
    // let rt = Runtime::new().unwrap();
    // let mut display = UnicornMini::new(&rt);

    // let dim = Dimensions::new(7,17);

    // let mut layers = vec![
    //     Layer::new(GREEN, &dim), 
    //     Layer::new(RED, &dim), 
    //     Layer::new(BLUE, &dim)
    // ];

    // let start = chrono::offset::Utc::now();
    // println!(" = {}",start.timestamp_millis());

    // loop {
    //     println!("  {}", (start- chrono::offset::Utc::now()).num_milliseconds() % 1000);

    //     let mut pxls = vec![BLACK; dim.num_pixels() as usize];

    //     for layer in layers.iter_mut() {
    //         for (id, &alive) in layer.current().iter().enumerate() {
    //             if alive { pxls[id] += layer.colour; }
    //         }
            
    //         if layer.has_repition() { 
    //             layer.reset(&dim); 
    //         } else { 
    //             layer.evolve(&dim) 
    //         }
    //     }

    //     for (idx, colour) in pxls.iter().enumerate() {
    //         display.set_idx(idx, colour )
    //     }

    //     display.flush();

    //     for idx in intersect(&layers, &dim) {
    //         for layer in layers.iter_mut() {
    //             layer.purge(idx);
    //         }
    //     }

    //     std::thread::sleep(Duration::from_millis(500));
    // }
}
