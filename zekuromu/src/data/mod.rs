pub mod operators;

use std::{collections::{HashMap, HashSet}, hash::Hash, num::ParseIntError};

use crate::data::operators::Reference;

// Explicitely constrains `Mapping` to only use Strings as keys.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct DataKey(String);

impl<T> From<T> for DataKey
where T: Into<String>
{
    fn from(value: T) -> Self {
        DataKey(value.into())
    }
}

impl TryInto<usize> for DataKey {
    type Error = ParseIntError;
    fn try_into(self) -> Result<usize, Self::Error> {
        self.0.parse()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataKeyPath(Vec<DataKey>);

impl DataKeyPath {
    pub fn empty() -> Self {
        DataKeyPath(vec![])
    }
}

impl From<DataKey> for DataKeyPath {
    fn from(value: DataKey) -> Self {
        DataKeyPath(vec![value])
    }
}

impl TryFrom<Reference> for DataKeyPath {
    type Error = ();

    // Terrible implementation, but that's a start ig
    // TODO: Supports escaped double quotes, escaped dots, and escaped escapes
    fn try_from(value: Reference) -> Result<Self, Self::Error> {
        let mut path = vec![];
        let mut waiting_for_double_quote = false;
        let mut current_entry: Vec<char> = vec![];

        for character in value.0.chars() {
            if character == '"' {
                if waiting_for_double_quote {
                    waiting_for_double_quote = false;
                } else {
                    waiting_for_double_quote = true;
                }
            } else {
                if character == '.' && !waiting_for_double_quote {
                    path.push(current_entry.drain(..).collect::<String>().into());
                } else {
                    current_entry.push(character);
                }
            }
        }

        path.push(current_entry.drain(..).collect::<String>().into());

        if waiting_for_double_quote {
            return Err(())
        }

        Ok(DataKeyPath(path))
    }
}

// Any serialized format should implement at least these.
// TODO: `Number` should be arbitrarily large in practice; left as a float for now.
#[derive(Default, Clone, Debug)]
pub enum RawData {
    #[default]
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Sequence(Vec<RawData>),
    Mapping(HashMap<DataKey, RawData>)
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

#[derive(Default, Clone, Debug)]
pub enum OperatorData {
    #[default]
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Operator(operators::Expr),
    Sequence(Vec<OperatorData>),
    Mapping(HashMap<DataKey, OperatorData>)
}
