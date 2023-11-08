use std::{marker::PhantomData, path::PathBuf, net::Ipv4Addr};

use disease::{Disease, Covid, Plague};
use output::{Output, Aggregator, Logger};
use serialise::Json;
use storage::DB;

use crate::storage::Disk;

mod storage {
    use std::{path::PathBuf, net::Ipv4Addr};

    pub trait Storage {
        fn store(&mut self, string: String);
    }
    
    pub struct Disk{
        pub path: PathBuf
    }
    impl Storage for Disk {
        fn store(&mut self, str: String) { println!("Storing {str} on the disk") }
    }
    
    pub struct DB{
        pub address: Ipv4Addr,
        pub user: String,
        pub pass: String,
    }
    impl Storage for DB {
        fn store(&mut self, str: String) { println!("Storing {} in the DB {}@{}", str, self.user, self.address) }
    }
}

mod serialise {
    use crate::output::DataPoint;

    pub trait Serialise{
        fn serialise(data: DataPoint) -> String;
    }
    pub struct Json{}
    impl Serialise for Json{
        fn serialise(data: DataPoint) -> String { format!("JSON({})", data.0) }
    }
    
    pub struct Toml{}
    impl Serialise for Toml{
        fn serialise(data: DataPoint) -> String { format!("TOML({})", data.0) }
    }
}

mod disease {
    use crate::output::DataPoint;

    pub trait Disease{
        fn get_disease_data_point(&self) -> DataPoint;
    }
    pub struct Covid{}
    impl Disease for Covid{
        fn get_disease_data_point(&self) -> DataPoint {
            DataPoint("Covid data point")
        }
    }

    pub struct Plague{}
    impl Disease for Plague{
        fn get_disease_data_point(&self) -> DataPoint {
            DataPoint("Plague data point")
        }
    }
}

mod output {
    use std::{marker::PhantomData, mem};

    use crate::{serialise::Serialise, storage::Storage};

    pub struct DataPoint(pub &'static str);

    pub trait Output{
        fn output(&mut self, dp: DataPoint);
        fn flush(&mut self) -> Option<Vec<String>>;
    }

    pub struct Aggregator<Se>
    where
        Se: Serialise
    {
        acc: Vec<String>,
        se: PhantomData<Se>
    }
    impl<Se> Aggregator<Se>
    where
        Se: Serialise
    {
        pub fn new() -> Aggregator<Se> {
            Aggregator { 
                acc: Vec::new(),
                se: PhantomData,
            }
        }
    }
    impl<Se> Output for Aggregator<Se>
    where
        Se: Serialise
    {
        fn output(&mut self, dp: DataPoint) {
            let serialised = Se::serialise(dp);
            println!("Aggregating: {:#?}", serialised);
            self.acc.push(serialised);
        }
        
        fn flush(&mut self) -> Option<Vec<String>> {
            let mut ret = Vec::new();
            mem::swap(&mut self.acc, &mut ret);
            Some(ret)
        }
    }

    pub struct Logger<St, Se>
        where 
            St: Storage,
            Se: Serialise 
    {
        pub st: St,
        pub se: PhantomData<Se>
    }

    impl<St, Se> Output for Logger<St, Se>
        where 
            St: Storage,
            Se: Serialise {
        fn output(&mut self, dp: DataPoint) {
            let serialised = <Se as Serialise>::serialise(dp);
            self.st.store(serialised)
        }

        fn flush(&mut self) -> Option<Vec<String>> {
            None
        }
    }
}

struct Program<D, O>
    where
        D: Disease,
        O: Output {
    disease: D,
    output: O
}

trait ProgramApi{
    fn run(&mut self);
    fn dump_output(&mut self) -> Option<Vec<String>>;
}
impl<D, O> ProgramApi for Program<D, O> 
    where
        D: Disease,
        O: Output {
    fn run(&mut self) {
        self.output.output(self.disease.get_disease_data_point());
        self.output.output(self.disease.get_disease_data_point());
        self.output.output(self.disease.get_disease_data_point());
    }
    fn dump_output(&mut self) -> Option<Vec<String>> {
        self.output.flush()
    }
}

fn main(){
    //These are loaded from a config
    // let disease_str = String::from("covid");
    let disease_str = String::from("plague");
    // let output_str = String::from("aggregated_json"); 
    let output_str = String::from("toml_to_db"); 
    // let output_str = String::from("json_to_disk"); 

    //Build the model
    let mut program = match disease_str.as_str() {
        "covid" => build(Covid{}, &output_str),
        "plague" => build(Plague{}, &output_str),
        _ => panic!("I don't do that disease")
    };

    program.run();

    println!("Flushed output: \n{:#?}", program.dump_output())
}

fn build<D>(disease: D, outputter: &str) -> Box<dyn ProgramApi> 
    where  
        D: Disease + 'static,
{
    match outputter {
        "aggregated_json" => {
            Box::new(Program{
                disease,
                output: Aggregator::<Json>::new()
            })
        },
        "json_to_disk" => {
            let outputter: output::Logger<Disk,Json> = {
                let disk = Disk {
                    path: PathBuf::new()
                };

                output::Logger {
                    st: disk,
                    se: PhantomData,
                }
            };
            Box::new(Program{
                disease,
                output: outputter,
            })
        },
        "toml_to_db" => {
            let outputter: Logger<DB,Json> = {
                let db = DB {
                    address: Ipv4Addr::new(127, 0, 0, 1),
                    user: "steve".into(),
                    pass: "1234password".into()
                };

                Logger {
                    st: db,
                    se: PhantomData,
                }
            };

            Box::new(Program{
                disease,
                output: outputter,
            })
        },
        _ => panic!("I don't know how to do that")
    }
}