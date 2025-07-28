pub mod operators;

use std::{borrow::Borrow, collections::{HashMap, HashSet}, hash::Hash};

// Explicitely constrains `Mapping` to only use Strings as keys.
pub type DataKey = String;

pub trait Traverseable<K> 
where
    K: Hash + Eq + ?Sized,
{
    fn get<Q>(&self, key: &Q) -> Option<&Self>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn get_path<Q>(&self, path: &[&Q]) -> Option<&Self>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if path.is_empty() {
            return Some(self)
        } else {
            let key = path[0];
            match self.get(key) {
                Some(inner) => inner.get_path(&path[1..]),
                None => None
            }
        }
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

impl Traverseable<String> for RawData {
    fn get<Q>(&self, key: &Q) -> Option<&Self>
    where
        String: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match self {
            RawData::Mapping(inner) => inner.get(key),
            _ => None,
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

impl Traverseable<DataKey> for OperatorData {
    fn get<Q>(&self, key: &Q) -> Option<&Self>
    where
        DataKey: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match self {
            OperatorData::Mapping(inner) => inner.get(key),
            _ => None,
        }
    }
}
