pub mod state;
pub mod environment;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EpiParams {
    pub beta_internal: f64,
    pub beta_external: f64,
    pub gamma: f64,
    pub sigma: f64,
}

impl EpiParams {
    pub fn default() -> Self {
        Self::new(0.5, 0.4, 1.0 / 6.0, 1.0 / 3.0)
    }

    pub fn new(beta_internal: f64, beta_external: f64, gamma: f64, sigma: f64) -> Self {
        EpiParams {
            beta_internal,
            beta_external,
            gamma,
            sigma,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let expected = EpiParams {
            beta_internal: 0.5,
            beta_external: 0.4,
            gamma: 1.0 / 6.0,
            sigma: 1.0 / 3.0,
        };

        assert_eq!(expected, EpiParams::default());
    }
}
