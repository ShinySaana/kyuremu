use std::{env, path::Path};

use zekuromu;
fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg_as_path = Path::new(&args[1]);

    println!("{:?}", zekuromu::raw::parse_file_as_raw_data(&first_arg_as_path).unwrap());
}
