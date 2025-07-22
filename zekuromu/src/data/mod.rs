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

#[derive(Default, Clone, Debug)]
pub enum OperatorData {
    #[default]
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Operator(operators::Expr),
    Sequence(Vec<OperatorData>),
    Mapping(HashMap<String, OperatorData>)
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

    pub fn into_operator_data(self) -> OperatorData {
        match self {
            RawData::Null => OperatorData::Null,
            RawData::Boolean(inner) => OperatorData::Boolean(inner),
            RawData::Number(inner) => OperatorData::Number(inner),
            RawData::String(inner) => {
                if let Some(expr) = operators::Expr::try_parse(&inner) {
                    OperatorData::Operator(expr)
                } else {
                    OperatorData::String(inner)
                }
            },
            RawData::Sequence(inner) => {
                let mut sequence = Vec::with_capacity(inner.len());
                for item in inner {
                    sequence.push(item.into_operator_data());
                }
                OperatorData::Sequence(sequence)
            },
            RawData::Mapping(inner) => {
                let mut mapping = HashMap::with_capacity(inner.len());
                for (inner_key, inner_value) in inner {
                    mapping.insert(inner_key, inner_value.into_operator_data());
                }
                OperatorData::Mapping(mapping)
            }
        }
    }
}
