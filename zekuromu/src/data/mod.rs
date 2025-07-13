use std::collections::HashMap;

#[derive(Debug)]

// Any serialized format should implement at least these.
// Explicitely constrains `Mapping` to only use Strings as keys.
// TODO: `Number` should be arbitrarily large in practice; left as a float for now.
pub enum RawData {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Sequence(Vec<RawData>),
    Mapping(HashMap<String, RawData>)
}
