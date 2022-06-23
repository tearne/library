use std::time::Duration;

use gol::dimensions::Dimensions;
use gol::layer::Layer;
use rgb::RGB8;
use tokio::runtime::Runtime;
use unicorn::pimoroni::unicorn::Unicorn;

static BLACK: RGB8 = RGB8::new(0,0,0);
static GREEN: RGB8 = RGB8::new(0, 220, 0);
static BLUE: RGB8 = RGB8::new(0, 0, 250);
static RED: RGB8 = RGB8::new(190,0,0);

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
    let mut display = Unicorn::new();

    let dim = Dimensions::new(16,16);

    let mut layers = vec![
        Layer::new(GREEN, &dim), 
        Layer::new(RED, &dim), 
        Layer::new(BLUE, &dim)
    ];

    for i in 0..dim.num_pixels() {
        display.set_idx(i as usize, &RED);
        display.flush();
    }
    for i in 0..dim.num_pixels() {
        display.set_idx(i as usize, &GREEN);
        display.flush();
    }
    for i in 0..dim.num_pixels() {
        display.set_idx(i as usize, &BLUE);
        display.flush();
    }
    std::thread::sleep(Duration::from_millis(1000));


    let interval_ms = 3000;
    let mut div = chrono::offset::Utc::now().timestamp_millis() / interval_ms;
    let mut rem = (chrono::offset::Utc::now().timestamp_millis() % interval_ms) as f64/ interval_ms as f64;

    loop {
        let div_new = chrono::offset::Utc::now().timestamp_millis() / interval_ms;
        rem = (chrono::offset::Utc::now().timestamp_millis() % interval_ms) as f64/ interval_ms as f64;
        if div_new != div {
            div = div_new;
            layers.iter_mut().for_each(|l|{
                l.evolve(&dim);
                if l.has_repition() { l.reset(&dim); }
            });
        }

        let mut pxls = vec![BLACK; dim.num_pixels() as usize];

        for layer in layers.iter_mut() {
            for px in 0..dim.num_pixels() {
                pxls[px] += layer.interpolate_history(px, rem); 
            }
        }

        for (idx, colour) in pxls.iter().enumerate() {
            display.set_idx(idx, colour )
        }

        display.flush();

        std::thread::sleep(Duration::from_millis(100))

        // for idx in intersect(&layers, &dim) {
        //     for layer in layers.iter_mut() {
        //         layer.purge(idx);
        //     }
        // }
    }
}
