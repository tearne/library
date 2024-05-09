use color_eyre::{
    eyre::{bail, eyre},
    Result, Section,
};
use std::fmt::Debug;

trait Validate: Debug {
    fn validate(&self) -> Result<()>;
}

#[derive(Debug)]
struct Chocolate(u8);

impl Validate for Chocolate {
    fn validate(&self) -> Result<()> {
        if self.0 > 10 {
            bail!("Too much chocolate ({})", self.0);
        } else if self.0 < 5 {
            bail!("Insufficient chocolate ({})", self.0);
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
struct Probability(f32);

impl Validate for Probability {
    fn validate(&self) -> Result<()> {
        if self.0 < 0.0 {
            bail!("Negative probability ({})", self.0);
        } else if self.0 > 1.0 {
            bail!("Probabilty exceeds 1.0 ({})", self.0);
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
enum MyTypes {
    C(Chocolate),
    P(Probability),
}
impl Validate for MyTypes {
    fn validate(&self) -> Result<()> {
        match self {
            MyTypes::C(c) => c.validate(),
            MyTypes::P(p) => p.validate(),
        }
    }
}

impl<T> Validate for Vec<T>
where
    T: Validate + Debug,
{
    fn validate(&self) -> Result<()> {
        let results: Vec<_> = self
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                v.validate().map_err(|e| {
                    eyre!("at index [{}]: {}", idx, e)
                })
            })
            .collect();

        if results.iter().all(|r| r.is_ok()) {
            return Ok(());
        }

        let err = results
            .into_iter()
            .filter(Result::is_err)
            .map(Result::unwrap_err)
            .fold(
                eyre!("Validation errors in Vector"),
                |acc, r| acc.note(r.to_string()),
            );

        Err(err)
    }
}

fn main() -> Result<()> {
    use MyTypes::*;
    color_eyre::install()?;

    vec![
        P(Probability(-0.1)),
        C(Chocolate(7)),
        P(Probability(0.2)),
        P(Probability(1.1)),
        P(Probability(0.9)),
        C(Chocolate(200)),
        C(Chocolate(3)),
    ]
    .validate()?;

    Ok(())
}
