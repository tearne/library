use super::cached::Cached;

pub struct Params{
    pub time_step: f32,
    tau_num_steps: f32,
    pub tau_sub_step: f32,
    pub beta: f32,
    pub gamma: f32,
    pub a: f32,
}

impl Params {
    fn new(time_step: f32,
        tau_num_steps: f32,
        beta: f32,
        gamma: f32,
        a: f32,
    ) -> Self {
        Params {
            time_step,
            tau_num_steps,
            tau_sub_step: time_step / tau_num_steps,
            beta,
            gamma,
            a,
        }
    }
    
}