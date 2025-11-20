use crate::{data::{DataKeyPath, RawOperatorData, operators::{Argument, Expr, StringLiteral}}, operators::{OperatorParsingErrorReason, OperatorPayload}};

#[derive(Debug, Clone)]
pub struct ExpectOperator {
    error_msg: StringLiteral
}

impl TryFrom<&Expr> for ExpectOperator {
    type Error = OperatorParsingErrorReason;

    fn try_from(value: &Expr) -> Result<Self, Self::Error> {
        if value.name.0 != "expect" {
            return Err(OperatorParsingErrorReason::NameDoesNotMatch)
        }

        if value.arguments.len() != 1 {
            return Err(OperatorParsingErrorReason::ArgumentsLengthDoesNotMatch)
        }

        let first_arg = value.arguments.get(0).unwrap();
        match first_arg {
            Argument::StringLiteral(inner) => Ok(ExpectOperator { error_msg: inner.clone() }),
            _ => Err(OperatorParsingErrorReason::ArgumentsTypesDoNotMatch)
        }
    }
}

impl OperatorPayload for ExpectOperator {
    fn execute(&self, _data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String> {
        Err(format!("At path '{}', expected a value. Message: '{}'", path, self.error_msg.0))
    }
}
