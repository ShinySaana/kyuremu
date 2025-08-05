use std::{env, path::Path};

use zekuromu::{self, data::{operators::Reference, DataKeyPath}};
fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg_as_path = Path::new(&args[1]);

    let data = zekuromu::raw::parse_file_as_raw_data(&first_arg_as_path).unwrap();
    let operator_hydrated = data.into_operator_data();



    // println!("{:?}", merged.into_operator_data());
}
