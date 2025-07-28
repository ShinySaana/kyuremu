use std::{env, path::Path};

use zekuromu::{self, data::Traverseable};
fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg_as_path = Path::new(&args[1]);
    let second_arg_as_path = Path::new(&args[2]);

    let data = zekuromu::raw::parse_file_as_raw_data(&first_arg_as_path).unwrap();
    let merge = zekuromu::raw::parse_file_as_raw_data(&second_arg_as_path).unwrap();
    let merged = data.recursive_merge(merge);

    println!("{:?}", merged.get_path(&["a"]));
    println!("{:?}", merged.into_operator_data());
}
