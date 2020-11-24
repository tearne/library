#![allow(unused)]

type Result<T> = std::result::Result<T, ()>;

// Things representing data
#[derive(Debug, PartialEq)]
struct Parameter { }
#[derive(Debug, PartialEq)]
struct Record { property: u32 }
#[derive(Debug, PartialEq, Clone)]
struct ProcessedRecord { }

// Components that work with data
struct Database {}
struct Config {}
struct Random {}
struct Processor {}

impl Database {
    fn get_input(&mut self, job_id: u64) -> Result<Record> {unimplemented!()}
    fn save_output(&mut self, out: &ProcessedRecord) -> Result<()> {unimplemented!()}
}
impl Config {
    fn get_parameter(&self, property: u32) -> Parameter {unimplemented!()}
}
impl Random {
    fn get_rnd(&mut self) -> f64 {unimplemented!()}
}
impl Processor {
    fn do_lengthy_processing(record: &Record, param: Parameter, rnd: f64) -> ProcessedRecord {unimplemented!()}
}

// This brings together the components to do work.
// Assume the components are already sufficiently tested, individually.
struct Plumbing {
    config: Config,
    database: Database,
    random: Random,
}
impl Plumbing {
    fn do_work(&mut self, job_id: u64) -> Result<ProcessedRecord> {
        //
        // Objective is to test this code which joins everything together.
        //
        let record = self.database.get_input(job_id)?;
        let parameter = self.config.get_parameter(record.property);
        let randomness = self.random.get_rnd();
        let processed = Processor::do_lengthy_processing(&record, parameter, randomness);

        Ok(processed)
    }
}


// In reality there wouldn't be a main here.  The `do_work` call
// below would be a dependency by some other part of my program,
// which would also need testing.  In other words, this isn't the
// 'top' of the application.
fn main() -> Result<()> {
    let mut plumbing = Plumbing {
        config: Config {},
        database: Database {},
        random: Random {},
    };

    let job_id = 1000;
    let output = plumbing.do_work(job_id)?;
    println!("Result of job {} is {:?}", job_id, output);
    Ok(())
}

//
// This is the best solution for testing the Plumbing that I've come up with so far
//

// Separate the operations from their concrete implementations
#[cfg_attr(test, mockall::automock)]
trait Operations {
    fn get_input(&mut self, job_id: u64) -> Result<Record>;
    fn save_output(&mut self, out: &ProcessedRecord) -> Result<()>;
    fn get_parameter(&self, property: u32) -> Parameter;
    fn get_rnd(&mut self) -> f64;
    fn slow_process(&mut self, record: &Record, parameter: Parameter, randomness: f64) -> ProcessedRecord;
}

struct ConcreteDependencies {
    config: Config,
    database: Database,
    random: Random,
}
impl Operations for ConcreteDependencies {
    // This is annoying to type out when there's lots of it ...
    // Slight worry that unnecessary calling of functions will have performance cost?
    fn get_input(&mut self, job_id: u64) -> Result<Record> {
        self.database.get_input(job_id)
    }
    fn save_output(&mut self, out: &ProcessedRecord) -> Result<()> {
        self.database.save_output(out)
    }
    fn get_parameter(&self, property: u32) -> Parameter {
        self.config.get_parameter(property)
    }
    fn get_rnd(&mut self) -> f64 {
        self.random.get_rnd()
    }
    fn slow_process(&mut self, record: &Record, parameter: Parameter, randomness: f64) -> ProcessedRecord {
        Processor::do_lengthy_processing(record, parameter, randomness)
    }
}

struct TestablePlumbing {}
impl TestablePlumbing {
    // impl trait to use specialisation rather than dyn, may be
    // faster if `do_work` is called a lot?
    fn do_work(job_id: u64, deps: &mut impl Operations) -> Result<ProcessedRecord> {
        let record = deps.get_input(job_id)?;
        let parameter = deps.get_parameter(record.property);
        let randomness = deps.get_rnd();

        let processed = deps.slow_process(&record, parameter, randomness);

        Ok(processed)
    }
}


// The main (caller) looks same as it did before ...
fn new_main() -> Result<()> {
    let mut dependencies = ConcreteDependencies {
        config: Config {},
        database: Database {},
        random: Random {},
    };

    let job_id = 1000;
    let output = TestablePlumbing::do_work(job_id, &mut dependencies)?;
    println!("Result of job {} is {:?}", job_id, output);
    Ok(())
}


// ... but now we can test 'do_work".
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_wiring() {
        let mut mock_ops = MockOperations::new();

        mock_ops.expect_get_input()
            .with(eq(1000))
            .return_once(|_| Ok(Record { property: 42}));

        mock_ops.expect_get_parameter()
            .with(eq(42))
            .return_once(|_| Parameter {} );

        mock_ops.expect_get_rnd()
            .returning(|| 0.923);

        let expected_result = ProcessedRecord {};
        let cloned_result = expected_result.clone();
        mock_ops.expect_slow_process()
            .with(
                eq(Record { property: 42 }),
                eq(Parameter {}),
                eq(0.923 )
            )
            .return_once(|_,_,_|  cloned_result);

        assert_eq!(
            expected_result,
            TestablePlumbing::do_work(1000, &mut mock_ops).unwrap()
        )
    }
}