use rand::rngs::ThreadRng;
use rand::Rng;
use rgb::RGB8;

pub struct Pixel {
        position: f64,
        step_size: f64,
        going_up: bool,
        target_colour: RGB8,
        current_colour: RGB8,
        adjust_step: u8,
    }
    impl Pixel {
        pub fn new(rng: &mut ThreadRng) -> Self {
            Pixel{
                position: rng.gen(),
                step_size: rng.gen::<f64>() * 0.03,
                going_up: rng.gen_bool(0.5), 
                target_colour: RGB8::new(rng.gen(), rng.gen(),rng.gen()),
                current_colour: RGB8::new(0,0,0),
                adjust_step: 1,
            }
        }
        
        pub fn evolve_and_get(&mut self) -> RGB8 {
            if self.going_up {
                self.position += self.step_size
            } else {
                self.position -= self.step_size
            }
    
            if self.position > 1.0 {
                self.position = 1.0;
                self.going_up = false;
            } else if self.position < 0.0 {
                self.position = 0.0;
                self.going_up = true;
            }

            fn step_towards(actual: &mut u8, target: &u8, adjust_step: u8) {
                if target != actual {
                    let diff = *target as f64 - *actual as f64;
                    let direction = diff / diff.abs();
                    *actual = (direction * (adjust_step as f64) + (*actual as f64)) as u8;
                }
            }

            step_towards(&mut self.current_colour.r, &self.target_colour.r, self.adjust_step);
            step_towards(&mut self.current_colour.g, &self.target_colour.g, self.adjust_step);
            step_towards(&mut self.current_colour.b, &self.target_colour.b, self.adjust_step);
    
            rgb::RGB8::new(
                ((self.current_colour.r as f64 * self.position) as u8).max(5),
                ((self.current_colour.g as f64 * self.position) as u8).max(5), 
                ((self.current_colour.b as f64 * self.position) as u8).max(5) 
            )
        }
    
        pub fn randomise(&mut self, rng: &mut ThreadRng) {
            self.step_size = rng.gen::<f64>() * 0.03;
            self.target_colour = RGB8::new(
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                rng.gen_range(0, 255)
            );
            self.adjust_step = rng.gen_range(1, 10);
        }
    }


// pub struct Pixel {
//     lower: [f64; 3],
//     upper: [f64; 3],
//     bright: f64,
//     position: f64,
//     step_size: f64,
//     going_up: bool,
// }
// impl Pixel {
//     pub fn new(rng: &mut ThreadRng) -> Self {
//         let mut lower = [0.0f64; 3];
//         let mut upper = [0.0f64; 3];
//         lower[0] = rng.gen_range(0.0, 255.0);
//         upper[0] = rng.gen_range(lower[0], 256.0);
//         lower[1] = rng.gen_range(0.0, 255.0);
//         upper[1] = rng.gen_range(lower[1], 256.0);
//         lower[2] = rng.gen_range(0.0, 255.0);
//         upper[2] = rng.gen_range(lower[2], 256.0);

//         Pixel{
//             lower, 
//             upper,
//             bright: 0.5,
//             position: rng.gen(),
//             step_size: rng.gen::<f64>() * 0.1,
//             going_up: rng.gen_bool(0.5), 
//         }
//     }
    
//     pub fn get_rgb(&self) -> RGB8 {
//         fn get_component(lower: f64, upper:f64, position: f64, bright: f64) -> u8 {
//             (((upper - lower) * position + lower ) * bright) as u8
//         }

//         rgb::RGB8::new(
//             get_component(self.lower[0], self.upper[0], self.position, self.bright), 
//             get_component(self.lower[1], self.upper[1], self.position, self.bright), 
//             get_component(self.lower[2], self.upper[2], self.position, self.bright),
//         )
//     }

//     pub fn step(&mut self) {
//         if self.going_up {
//             self.position += self.step_size
//         } else {
//             self.position -= self.step_size
//         }

//         if self.position > 1.0 {
//             self.position = 1.0;
//             self.going_up = false;
//         } else if self.position < 0.0 {
//             self.position = 0.0;
//             self.going_up = true;
//         }
//     }

//     pub fn randomise(&mut self, rng: ThreadRng) {
//         self.lower[0] = rng.gen_range(0.0, 255.0);
//         self.upper[0] = rng.gen_range(self.lower[0], 256.0);
//         self.lower[1] = rng.gen_range(0.0, 255.0);
//         self.upper[1] = rng.gen_range(self.lower[1], 256.0);
//         self.lower[2] = rng.gen_range(0.0, 255.0);
//         self.upper[2] = rng.gen_range(self.lower[2], 256.0);
//     }
// }

