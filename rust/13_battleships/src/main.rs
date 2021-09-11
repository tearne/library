use types::{Boat, Orientation, Position};
use unicorn::{keyboard, pimoroni::unicorn::Unicorn, RGB};

const KEY_UP: u16 = 103;
const KEY_DOWN: u16 = 108;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;

mod types;

#[tokio::main]
async fn main() {
    let mut rx = keyboard::grab_all_keyboards();
    let mut display = Unicorn::new();

    fn fresh_grid() -> [[RGB; 16]; 16] {
        [[RGB::BLACK; 16]; 16]
    }
    fn render(display: &mut Unicorn, grid: &[[RGB; 16]; 16]) {
        for x in 0..16usize {
            for y in 0..16usize {
                display.set_xy(x, y, &grid[x][y]);
            }
        }
    }

    let mut boat = Boat {
        orientation: Orientation::LEFT,
        position: Position { x: 10, y: 5 },
        size: 6,
    };

    let c1 = RGB::new(250, 250, 0);
    let c2 = RGB::new(250, 0, 250);
    let c3 = RGB::new(0, 250, 250);

    let mut grid = fresh_grid();

    while let Some(e) = rx.recv().await {
        if e.value == 1 {
            println!("--> {:?}", e);
            grid = fresh_grid();

            match e.code {
                2 => grid[1][6] = c1,
                3 => grid[2][5] = c2,
                4 => grid[3][4] = c2,
                5 => grid[4][3] = c1,
                6 => grid[5][2] = c2,
                7 => grid[6][1] = c1,
                KEY_UP => boat.set_orientation(Orientation::UP),
                KEY_DOWN => boat.set_orientation(Orientation::DOWN),
                KEY_LEFT => boat.set_orientation(Orientation::LEFT),
                KEY_RIGHT => boat.set_orientation(Orientation::RIGHT),
                _ => (),
            }

            boat.render_pixels().iter().for_each(|p| {
                grid[p.x()][p.y()] = p.colour;
            });

            render(&mut display, &grid);
            display.flush();
        }
    }
}
