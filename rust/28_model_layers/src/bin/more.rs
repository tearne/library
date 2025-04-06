use std::marker::PhantomData;

use framework::LayeredModel;
use model_as_layer::IntegratedModel;
use my_model::{Dependencies, ModelState, Movement};

pub fn main() {
    let model_a_movements = vec![Movement{}];
    let model_b_movements = vec![Movement{}];

    let model_a = IntegratedModel::<f64> {
        state: Default::default(),
        movements: &model_a_movements,
        deps: Dependencies{},
    };

    let model_b = IntegratedModel::<f64> {
        state: Default::default(),
        movements: &model_b_movements,
        deps: Dependencies{},
    };

    // let model_c = IntegratedModel::<u8> {
    //     state: Default::default(),
    //     movements: &model_b_movements,
    //     deps: Dependencies{},
    // };

    // enum MyModelTypes<'a>{
    //     EightBit(IntegratedModel::<'a, u8>),
    //     Float(IntegratedModel::<'a, f64>)
    // }
    // let both_models = vec![
    //     MyModelTypes::Float(model_a), 
    //     MyModelTypes::Float(model_b), 
    //     MyModelTypes::EightBit(model_c)
    // ];


    let both_models = vec![model_a, model_b];

    let mut joint_model = LayeredModel {
        phantom: Default::default(),
        models: both_models,
    };

    joint_model.run();

}

//==============================================

mod my_model {
    use std::marker::PhantomData;

    use crate::framework::RunStatus;

    #[derive(Default)]
    pub struct ModelState<T>{
        pub phantom: PhantomData<T>
    }
    pub struct Dependencies{}
    pub struct Movement{}
    
    impl<T> ModelState<T> {
        pub fn step(&mut self, movs: &[Movement]) -> RunStatus { RunStatus::More }
        pub fn current_step(&self) -> u64 { 0 }
        pub fn inject(&mut self, vec: &[(usize, f32)])  { }
        pub fn report(&self) -> Vec<(usize, f32)>  { Vec::new() }
        pub fn finish(&self)  { }
        pub fn other_things_too(&self) -> () { }
    }
}

//==============================================

mod framework {
    use core::num;
    use std::marker::PhantomData;

    use itertools::Itertools;

    #[derive(Eq, PartialEq)]
    pub enum RunStatus{
        More, Done
    }
    
    pub trait ModelLayer<I>{
        // I: Layer interaction data
        fn step(&mut self) -> RunStatus;
        fn current_step(&self) -> u64;
        fn inject(&mut self, vec: &[I]);
        fn report(&self) -> Vec<I>;
        fn finish(&self);
    }

    pub struct LayeredModel<I, M: ModelLayer<I>>{
        pub phantom: PhantomData<I>,
        pub models: Vec<M>,
    }
    impl<I,M: ModelLayer<I>> LayeredModel<I, M> {
        pub fn run(&mut self) {
            let num_models = self.models.len();
            let mut time_step = -1;

            loop {
                time_step += 1;
                println!("Time step {}", time_step);
                
                let is_end = (0..num_models).find(|&i|{
                    let model = &mut self.models[i];
                    let run_status = model.step();
                    
                    let report = model.report();

                    (0..num_models).filter(|j|*j != i)
                        .for_each(|j|{
                            self.models[j].inject(&report);
                        });

                    run_status == RunStatus::Done
                });
                if is_end.is_some() || time_step == 9 { break }
            }
        }
    }
}


//==============================================

mod model_as_layer {
    use crate::{framework::{ModelLayer, RunStatus}, my_model::{Movement, Dependencies, ModelState}};

    pub struct IntegratedModel<'a, T>{
        pub state: ModelState<T>,
        pub movements: &'a Vec<Movement>,
        pub deps: Dependencies,
    }
    
    impl<'a, T> ModelLayer<(usize, f32)> for IntegratedModel<'a, T> {
        fn step(&mut self) -> RunStatus {
            let chunk_of_movements = &self.movements[0..0];
            self.state.step(chunk_of_movements)
        }
    
        fn current_step(&self) -> u64 {
            self.state.current_step()
        }
    
        fn inject(&mut self, data: &[(usize, f32)]) {
            self.state.inject(data);
        }
    
        fn report(&self) -> Vec<(usize, f32)> {
            self.state.report()
        }
    
        fn finish(&self) {
            self.state.finish();
        }
    }
}
