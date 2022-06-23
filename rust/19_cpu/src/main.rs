use std::{time::Duration, ops::Range};

use rand::Rng;
use rgb::RGB8;
use unicorn::pimoroni::unicorn::Unicorn;

static BLACK: RGB8 = RGB8::new(0,0,0);
static GREEN: RGB8 = RGB8::new(0, 220, 0);
static BLUE: RGB8 = RGB8::new(0, 0, 250);
static RED: RGB8 = RGB8::new(190,0,0);

fn main() {
    let mut display = Unicorn::new();

    for i in 0..16*16 {
        display.set_idx(i as usize, &RED);
        display.flush();
    }
    for i in 0..16*16 {
        display.set_idx(i as usize, &GREEN);
        display.flush();
    }
    for i in 0..16*16 {
        display.set_idx(i as usize, &BLUE);
        display.flush();
    }
    std::thread::sleep(Duration::from_millis(500));

    let mut collector = psutil::cpu::CpuTimesPercentCollector::new().unwrap();
    let mut rng = rand::thread_rng();
    let _ = collector.cpu_times_percent_percpu();

    fn rect_ids(xs: Range<usize>, yx: Range<usize>) -> Vec<usize> {
        let mut res = Vec::with_capacity(256);
        for x in xs {
            for y in yx.clone() {
                res.push(y * 16 + x);
            }
        }
        res
    }

    let quarters = vec![
        rect_ids(0..8, 0..8),
        rect_ids(8..16, 0..8),
        rect_ids(0..8, 8..16),
        rect_ids(8..16, 8..16),
    ];

    loop {
        let per_cpu = collector.cpu_times_percent_percpu().unwrap();

        let loads: Vec<f64> = per_cpu.iter().map(|cpu|{
            ((100.0 - cpu.idle()) / 100.0) as f64
        }).collect();

        for (cpu_id, quarter) in quarters.iter().enumerate() {
            for px in quarter {
                let load = loads[cpu_id];
                if rng.gen_bool(load as f64) {
                    display.set_idx(*px, &RED);
                } else {
                    display.set_idx(*px, &BLACK);
                }
            }
        }
        display.flush();
        
        std::thread::sleep(Duration::from_millis(1000));
    }
}


