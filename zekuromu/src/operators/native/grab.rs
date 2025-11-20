use crate::{data::{DataKeyPath, RawOperatorData, operators::{Argument, Expr, Reference}}, operators::{OperatorParsingErrorReason, OperatorPayload}};

#[derive(Debug, Clone)]
pub struct GrabOperator {
    reference: Reference
}

impl TryFrom<&Expr> for GrabOperator {
    type Error = OperatorParsingErrorReason;

    fn try_from(value: &Expr) -> Result<Self, Self::Error> {
        if &value.name.0 != "grab" {
            return Err(OperatorParsingErrorReason::NameDoesNotMatch)
        }

        if value.arguments.len() != 1 {
            return Err(OperatorParsingErrorReason::ArgumentsLengthDoesNotMatch)
        }

        let first_arg = value.arguments.get(0).unwrap();
        match first_arg {
            Argument::Reference(inner) => Ok(GrabOperator { reference: inner.clone() }),
            _ => Err(OperatorParsingErrorReason::ArgumentsTypesDoNotMatch)
        }    }
}


impl OperatorPayload for GrabOperator {
    fn execute(&self, data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String> {
        let _ = data;
        let _ = path;
        Err("Unimplemented".into())
    }
}
