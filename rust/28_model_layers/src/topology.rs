fn main() {

    let model_a = ModelA{ sites: Vec::new() };
    let model_b = ModelB{ agents: Vec::new() };

    let topology: Topology<ModelA, ModelB> = todo!(); 

    loop {
        let a = model_a.get_infections();
        let b = model_b.get_infections();
    }
}

trait Topology<X, Y, Q, R, A: Model<X, Y> ,B: Model<Q,R>> {
    fn map(&self, items: Vec<(X,Y)>) -> Vec<(Q,R)>;
}

trait Model<Idx, Inf> {
    fn get_infections(&self) -> Vec<(Idx, Inf)>;
    fn apply_infections(&mut self, inf: Vec<(Idx, Inf)>);
}

struct ModelA {
    sites: Vec<f32>
}

impl Model<usize, f32> for ModelA {
    fn get_infections(&self) -> Vec<(usize, f32)> {
        self.sites
            .iter()
            .enumerate()
            .filter_map(|(idx, &prevalence)|{
                if prevalence > 0.0 {
                    Some((idx, prevalence))
                } else {
                    None
                }
            })
            .collect()
    }

    fn apply_infections(&mut self, inf: Vec<(usize, f32)>) {
        todo!()
    }
}

struct ModelB {
    agents: Vec<bool>
}

impl Model<usize, bool> for ModelB {
    fn get_infections(&self) -> Vec<(usize, bool)> {
        self.agents
            .iter()
            .enumerate()
            .filter_map(|(idx, &infected)|{
                if infected { Some((idx, true)) }
                else { None }
            })
            .collect()
    }

    fn apply_infections(&mut self, inf: Vec<(usize, bool)>) {
        todo!()
    }
}
