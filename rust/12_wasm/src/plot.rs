use eframe::egui::{Response, Ui, Widget, plot::{Legend, Plot, Points, Value, Values}};
use rand::Rng;

pub struct MyPlot {
    dots: Vec<Value>,
}
impl MyPlot {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        MyPlot {
            dots: (0..500).map(|_| Value::new(rng.gen::<i32>(), rng.gen::<i32>())).collect(),
        }
    }

    // fn min_max(&self) -> (Value, Value) {
    //     let mut min: Value = Value::new(0,0);
    //     let mut max: Value = Value::new(0,0);

    //     for dot in self.dots.iter() {
    //         if dot.x < min.x {
    //             min.x = dot.x 
    //         }
    //         else if dot.x > max.x {
    //             max.x = dot.x
    //         }

    //         if dot.y < min.y {
    //             min.y = dot.y 
    //         }
    //         else if dot.y > max.y {
    //             max.y = dot.y
    //         }
    //     }

    //     (min, max)
    // }
}

impl Widget for &mut MyPlot {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut plot = Plot::new("lines_demo")
            .points(Points::new(Values::from_values(self.dots.clone())))
            .legend(Legend::default());
        plot = plot.view_aspect(1.0);
        
        ui.add(plot)
    }
}