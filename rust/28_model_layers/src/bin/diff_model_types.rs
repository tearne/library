use framework::{ModelSandwich, RunStatus};
use my_integrated_model::IntegratedModel;
use my_model::{Dependencies, Movement};

mod my_model {
    use crate::framework::RunStatus;

    #[derive(Default)]
    pub struct ModelState<T>{
        pub phantom: std::marker::PhantomData<T>
    }
    pub struct Dependencies{}
    pub struct Movement{}
    
    impl<T> ModelState<T> {
        // These are the actual implementations
        pub fn step(&mut self, movs: &[Movement], deps: &Dependencies) -> RunStatus { RunStatus::More }
        pub fn current_step(&self) -> u64 { 0 }
        pub fn inject(&mut self, inf: &[T], deps: &Dependencies)  { }
        pub fn report(&self, deps: &Dependencies) -> Vec<T>  { Vec::new() }
        pub fn finish(&self, deps: &Dependencies)  { }
        pub fn other_things_too(&self, deps: &Dependencies) -> () { }
    }
}

pub fn main() {
    let movements = vec![Movement{}];

    let float_model = IntegratedModel::<f64> {
        state: Default::default(),
        movements: &movements,
        deps: Dependencies{},
    };

    let eight_model = IntegratedModel::<u8> {
        state: Default::default(),
        movements: &movements,
        deps: Dependencies{},
    };

    let isize_model = IntegratedModel::<isize> {
        state: Default::default(),
        movements: &movements,
        deps: Dependencies{},
    };

    struct MySandwich<'a>{
        float_model: IntegratedModel<'a, f64>,
        eight_model: IntegratedModel<'a, u8>,
        isize_model: IntegratedModel<'a, isize>,
    }
    
    impl<'a> ModelSandwich for MySandwich<'a> {
        fn step_all_models(&mut self) -> framework::RunStatus {
            println!("Step \"float\" model");
            let run_status_a = self.float_model.step();
            let report = self.float_model.report();
            self.eight_model.inject(&report);
            self.isize_model.inject(&report);

            println!("Step \"eight\" model");
            let run_status_b = self.eight_model.step();
            let report = self.eight_model.report();
            self.float_model.inject(&report);
            self.isize_model.inject(&report);

            println!("Step \"isize\" model");
            let run_status_c = self.isize_model.step();
            let report = self.isize_model.report();
            self.eight_model.inject(&report);
            self.float_model.inject(&report);

            if (run_status_a == RunStatus::Done) || 
                (run_status_b == RunStatus::Done) || 
                (run_status_c == RunStatus::Done) {
                RunStatus::Done
            } else {
                RunStatus::More
            }
        }
    }

    let mut my_sandwich = MySandwich{
        float_model,
        eight_model,
        isize_model,
    };

    my_sandwich.run();

}

//==============================================

mod framework {
    #[derive(Eq, PartialEq)]
    pub enum RunStatus{
        More, Done
    }
    
    pub trait ModelSandwich{
        fn step_all_models(&mut self) -> RunStatus;
        
        fn run(&mut self) {
            let mut time_step = -1;

            loop {
                time_step += 1;
                println!("*** Time step {}", time_step);

                let run_status = self.step_all_models();
                if run_status == RunStatus::Done || time_step == 9 { break }
            }
        }
    }
}

//==============================================

mod my_integrated_model {
    use std::borrow::Cow;

    use crate::{framework::RunStatus, my_model::{Movement, Dependencies, ModelState}};

    #[derive(Clone)]
    pub enum InteractionData {
        VecOfFloats(Vec<f64>),
        VecOfEights(Vec<u8>),
        VecOfIsize(Vec<isize>),
    }
    impl InteractionData {
        fn to_f64(&self) -> Cow<Vec<f64>> {
            match self {
                InteractionData::VecOfFloats(items) => Cow::Borrowed(items),
                InteractionData::VecOfEights(items) => 
                    Cow::Owned(items.into_iter().map(|&v|v as f64).collect()),
                InteractionData::VecOfIsize(items) =>
                    Cow::Owned(items.into_iter().map(|&v|v as f64).collect()),
            }
        }

        fn to_u8(&self) -> Cow<Vec<u8>> {
            match self {
                InteractionData::VecOfFloats(items) => 
                    Cow::Owned(
                        items.into_iter()
                            .map(|&v|num_traits::cast::<f64, u8>(v).unwrap())
                            .collect()
                    ),
                InteractionData::VecOfEights(items) => Cow::Borrowed(items),
                InteractionData::VecOfIsize(items) =>
                    Cow::Owned(
                        items.into_iter()
                            .map(|&v| 
                                if v<0 {0u8} 
                                else { num_traits::cast::<isize, u8>(v).unwrap() }
                            )
                            .collect()
                    ),
            }
        }

        fn to_isize(&self) -> Cow<Vec<isize>> {
            match self {
                InteractionData::VecOfFloats(items) => Cow::Owned(
                    items.into_iter()
                        .map(|&v|num_traits::cast::<f64, isize>(v).unwrap())
                        .collect()
                ),
                InteractionData::VecOfEights(items) => Cow::Owned(
                    items.into_iter()
                        .map(|&v|v as isize)
                        .collect()
                ),
                InteractionData::VecOfIsize(items) => Cow::Borrowed(items),
            }
        }
    }

    pub struct IntegratedModel<'a, T>{
        pub state: ModelState<T>,
        pub movements: &'a Vec<Movement>,
        pub deps: Dependencies,
    }
    
    impl<'a> IntegratedModel<'a, f64> {
        pub fn step(&mut self) -> RunStatus {
            let chunk_of_movements = &self.movements[0..0];
            self.state.step(chunk_of_movements, &self.deps)
        }
    
        fn current_step(&self) -> u64 {
            self.state.current_step()
        }
    
        pub fn inject(&mut self, data: &InteractionData) {
            self.state.inject(&data.to_f64(), &self.deps);
        }
    
        pub fn report(&self) -> InteractionData {
            InteractionData::VecOfFloats(self.state.report(&self.deps))
        }
    
        pub fn finish(&self) {
            self.state.finish(&self.deps);
        }
    }

    impl<'a> IntegratedModel<'a, u8> {
        pub fn step(&mut self) -> RunStatus {
            let chunk_of_movements = &self.movements[0..0];
            self.state.step(chunk_of_movements, &self.deps)
        }
    
        pub fn current_step(&self) -> u64 {
            self.state.current_step()
        }
    
        pub fn inject(&mut self, data: &InteractionData) {
            self.state.inject(&data.to_u8(), &self.deps);
        }
    
        pub fn report(&self) -> InteractionData {
            InteractionData::VecOfEights(self.state.report(&self.deps))
        }
    
        pub fn finish(&self) {
            self.state.finish(&self.deps);
        }
    }

    impl<'a> IntegratedModel<'a, isize> {
        pub fn step(&mut self) -> RunStatus {
            let chunk_of_movements = &self.movements[0..0];
            self.state.step(chunk_of_movements, &self.deps)
        }
    
        pub fn current_step(&self) -> u64 {
            self.state.current_step()
        }
    
        pub fn inject(&mut self, data: &InteractionData) {
            self.state.inject(&data.to_isize(), &self.deps);
        }
    
        pub fn report(&self) -> InteractionData {
            InteractionData::VecOfIsize(self.state.report(&self.deps))
        }
    
        pub fn finish(&self) {
            self.state.finish(&self.deps);
        }
    }
}

