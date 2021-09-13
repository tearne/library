use types::Position;
use unicorn::{keyboard, pimoroni::unicorn::Unicorn};
use crate::types::{Game, Renderable, Selector};

use crate::types::{Path, Tower, World};

const KEY_UP: u16 = 103;
const KEY_DOWN: u16 = 108;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;

mod types;

#[tokio::main]
async fn main() {
    let mut rx = keyboard::grab_all_keyboards();
    let mut display = Unicorn::new();
    
    let mut world = World {
        path: Path::build(
            vec![
                Position::new(0, 2),
                Position::new(13, 2),
                Position::new(13, 5),
                Position::new(2, 5),
                Position::new(2, 8),
                Position::new(13, 8),
                Position::new(13, 11),
                Position::new(2, 11)
            ]),
        towers: vec![Tower::new(0), Tower::new(1)]
    };
    let mut game = Game::new(world);

    game.selector = Some(Selector{
        holding: None,
        position: Position::new(3,5),
    });
    game.render_layer().send_to_display(&mut display);
    display.flush();


    while let Some(e) = rx.recv().await {
        if e.value == 1 {
            println!("--> {:?}", e);

            match e.code {
                // 2 => grid[1][6] = c1,
                // 3 => grid[2][5] = c2,
                // 4 => grid[3][4] = c2,
                // 5 => grid[4][3] = c1,
                // 6 => grid[5][2] = c2,
                // 7 => grid[6][1] = c1,
                KEY_UP => 
                    game.selector.as_mut().unwrap().position.y_shift(-1),
                KEY_DOWN => 
                    game.selector.as_mut().unwrap().position.y_shift(1),
                KEY_LEFT => 
                    game.selector.as_mut().unwrap().position.x_shift(-1),
                KEY_RIGHT => 
                    game.selector.as_mut().unwrap().position.x_shift(1),
                _ => (),
            }

            game.render_layer().send_to_display(&mut display);
            display.flush();
        }
    }
}
