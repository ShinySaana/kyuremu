use std::{env, path::Path};

use zekuromu::{self, data::{operators::Reference, DataKeyPath, OperatorData}};
fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg_as_path = Path::new(&args[1]);

    let data: zekuromu::data::RawData = zekuromu::raw::parse_file_as_raw_data(&first_arg_as_path).unwrap();
    let operator_hydrated: zekuromu::data::RawOperatorData = data.into_raw_operator_data();
    println!("{:?}", operator_hydrated);

    let operator_data: OperatorData = operator_hydrated.try_into().unwrap();
    println!("{:?}", operator_data);
}
