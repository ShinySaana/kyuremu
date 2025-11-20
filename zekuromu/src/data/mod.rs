pub mod operators;

use std::{collections::{HashMap, HashSet}, fmt::Display, hash::Hash, num::ParseIntError};

use crate::{data::{operators::Reference}, operators::{Operator, OperatorParsingError}};

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

impl Display for DataKeyPath {
    // God this is terrible
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.0.iter()
            .map(|item| item.0.clone())
            .collect::<Vec<String>>()
            .join("\".\"");
        write!(f, "\"{}\"", joined)
    }
}

impl From<DataKey> for DataKeyPath {
    fn from(value: DataKey) -> Self {
        DataKeyPath(vec![value])
    }
}

impl TryFrom<Reference> for DataKeyPath {
    type Error = ();
    fn try_from(value: Reference) -> Result<Self, Self::Error> {
        let mut builder = Vec::with_capacity(value.0.len());
        for element in value.0 {
            let data_key: DataKey = element.into();
            builder.push(data_key);
        }
        Ok(DataKeyPath(builder))
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

    pub fn into_raw_operator_data(self) -> RawOperatorData {
        match self {
            RawData::Null => RawOperatorData::Null,
            RawData::Boolean(inner) => RawOperatorData::Boolean(inner),
            RawData::Number(inner) => RawOperatorData::Number(inner),
            RawData::String(inner) => {
                if let Some(expr) = operators::Expr::try_parse(&inner) {
                    RawOperatorData::RawOperator(expr)
                } else {
                    RawOperatorData::String(inner)
                }
            },
            RawData::Sequence(inner) => {
                let mut sequence = Vec::with_capacity(inner.len());
                for item in inner {
                    sequence.push(item.into_raw_operator_data());
                }
                RawOperatorData::Sequence(sequence)
            },
            RawData::Mapping(inner) => {
                let mut mapping = HashMap::with_capacity(inner.len());
                for (inner_key, inner_value) in inner {
                    mapping.insert(inner_key, inner_value.into_raw_operator_data());
                }
                RawOperatorData::Mapping(mapping)
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum RawOperatorData {
    #[default]
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    RawOperator(operators::Expr),
    Sequence(Vec<RawOperatorData>),
    Mapping(HashMap<DataKey, RawOperatorData>)
}

impl TryInto<OperatorData> for RawOperatorData {
    type Error = OperatorParsingError;

    fn try_into(self) -> Result<OperatorData, Self::Error> {
        match self {
            RawOperatorData::Null => Ok(OperatorData::Null),
            RawOperatorData::Boolean(inner) => Ok(OperatorData::Boolean(inner)),
            RawOperatorData::Number(inner) => Ok(OperatorData::Number(inner)),
            RawOperatorData::String(inner) => Ok(OperatorData::String(inner)),
            RawOperatorData::RawOperator(inner) => crate::operators::native::NativeOperator::try_parsing_operator(&inner).map( |it| OperatorData::Operator(it)),
            RawOperatorData::Sequence(inner) => {
                let mut sequence = Vec::with_capacity(inner.len());
                for item in inner {
                    let intoed = item.try_into()?;
                    sequence.push(intoed);
                }
                Ok(OperatorData::Sequence(sequence))
            },
            RawOperatorData::Mapping(inner) => {
                let mut mapping = HashMap::with_capacity(inner.len());
                for (inner_key, inner_value) in inner {
                    let intoed = inner_value.try_into()?;
                    mapping.insert(inner_key, intoed);
                }
                Ok(OperatorData::Mapping(mapping))
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
    Operator(Operator),
    Sequence(Vec<OperatorData>),
    Mapping(HashMap<DataKey, OperatorData>)
}
