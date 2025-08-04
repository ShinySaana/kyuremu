use std::{env, path::Path};

use zekuromu::{self, data::{operators::Reference, DataKeyPath}};
fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg_as_path = Path::new(&args[1]);
    let second_arg_as_path = Path::new(&args[2]);

    let data = zekuromu::raw::parse_file_as_raw_data(&first_arg_as_path).unwrap();
    let merge = zekuromu::raw::parse_file_as_raw_data(&second_arg_as_path).unwrap();
    let mut merged = data.recursive_merge(merge);

    let path = DataKeyPath::try_from(Reference("abc.d.e..f.\"g.h\".ij".into()));
    println!("{:?}", path);

    // println!("{:?}", merged.get_path(&["a"]));

    // merged.set_path(&[&String::from("new")], &zekuromu::data::RawData::Null);

    println!("{:?}", merged.into_operator_data());
}
