use std::{collections::HashMap, io::BufReader};

use crate::data::RawData;

pub fn parse_as_raw_data<R: std::io::Read>(buffer: &mut BufReader<R>) -> super::ResultDataParsing {
    let raw_yaml: serde_yaml::Value = serde_yaml::from_reader(buffer).unwrap();
    parse_yaml_value(&raw_yaml)
}

fn parse_yaml_value(value: &serde_yaml::Value) -> Result<RawData, String> {
    match value {
        serde_yaml::Value::Null => Ok(RawData::Null),
        serde_yaml::Value::Bool(inner) => Ok(RawData::Boolean(*inner)),
        serde_yaml::Value::Number(inner) => {
            let cast_as_f64 = inner.as_f64()
                .ok_or("Could not parse number as f64.".to_string())?;
            Ok(RawData::Number(cast_as_f64))
        },
        serde_yaml::Value::String(inner) => Ok(RawData::String(inner.clone())),
        serde_yaml::Value::Sequence(inner) => {
            let mut sequence = Vec::with_capacity(inner.len());
            for item in inner {
                sequence.push(parse_yaml_value(item)?);
            }
            Ok(RawData::Sequence(sequence))
        }
        serde_yaml::Value::Mapping(inner) => {
            let mut mapping = HashMap::with_capacity(inner.len());
            for (inner_key, inner_value) in inner {
                let mapping_key =  match inner_key {
                    serde_yaml::Value::String(inner_key_string) => inner_key_string.clone(),
                    _ => return Err("Strings are only allowed as keys.".to_string())
                };

                let mapping_value = parse_yaml_value(inner_value)?;

                mapping.insert(mapping_key.into(), mapping_value);
            }
            Ok(RawData::Mapping(mapping))
        }
        serde_yaml::Value::Tagged(_) => Err("Tagged values are not supported".to_string()),
    }
}
