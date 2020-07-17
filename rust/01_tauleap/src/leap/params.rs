
use crate::epi::EpiParams;
#[derive(Copy, Clone, Debug)]
pub struct Params {
    pub time_step: f64,
    pub tau_sub_step: f64,
    pub epi_params: EpiParams,
}

impl Params {
    pub fn new(time_step: f64, tau_num_steps: i64, epi_params: EpiParams) -> Self {
        Params {
            time_step,
            tau_sub_step: time_step / (tau_num_steps as f64),
            epi_params,
        }
    }
}
