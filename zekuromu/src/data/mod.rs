pub mod operators;

use std::collections::{HashMap, HashSet};

// Any serialized format should implement at least these.
// Explicitely constrains `Mapping` to only use Strings as keys.
// TODO: `Number` should be arbitrarily large in practice; left as a float for now.
#[derive(Default, Clone, Debug)]
pub enum RawData {
    #[default]
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Sequence(Vec<RawData>),
    Mapping(HashMap<String, RawData>)
}

impl RawData {
    pub fn recursive_merge(self, other: RawData) -> RawData {
        match self {
            RawData::Mapping(mut self_mapping) => {
                if let RawData::Mapping(mut other_mapping) = other {
                    let keys: HashSet<_> = HashSet::from_iter(self_mapping
                        .keys()
                        .cloned()
                        .chain(
                            other_mapping
                            .keys()
                            .cloned()
                        ));

                    for key in keys {
                        if let Some(other_inner) = other_mapping.remove(&key) {
                            if let Some(self_inner) = self_mapping.remove(&key) {
                                self_mapping.insert(
                                    key.clone(),
                                    self_inner.recursive_merge(other_inner.clone())
                                );
                            } else {
                                self_mapping.insert(
                                    key.clone(),
                                    other_inner
                                );
                            }
                        }
                    }

                    RawData::Mapping(self_mapping)
                } else {
                    other
                }
            },
            _ => other
        }
    }
}

