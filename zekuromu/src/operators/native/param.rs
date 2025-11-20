use crate::{data::{DataKeyPath, OperatorData, operators::{Argument, Expr, StringLiteral}}, operators::{OperatorExecutionErrorReason, OperatorExecutionErrorResult, OperatorParsingErrorReason, OperatorPayload, OperatorPriority, OperatorPriorityRank}};

#[derive(Debug, Clone)]
pub struct ParamOperator {}

impl TryFrom<&Expr> for ParamOperator {
    type Error = OperatorParsingErrorReason;

    fn try_from(value: &Expr) -> Result<Self, Self::Error> {
        if value.name.0 != "param" {
            return Err(OperatorParsingErrorReason::NameDoesNotMatch)
        }

        if value.arguments.len() != 0 {
            return Err(OperatorParsingErrorReason::ArgumentsLengthDoesNotMatch)
        }

        Ok(ParamOperator {})
    }
}

impl OperatorPayload for ParamOperator {
    fn execute(&self, _data: &mut OperatorData, path: &DataKeyPath) -> OperatorExecutionErrorResult {
        Err(OperatorExecutionErrorReason::OtherError(
            format!("At path '{}', expected a parameter to be overriden", path)
        ))
    }
}

impl OperatorPriority for ParamOperator {
    fn priority(&self) -> OperatorPriorityRank {
        OperatorPriorityRank::AfterFirst
    }
}
