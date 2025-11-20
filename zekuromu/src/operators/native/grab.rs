use crate::{data::{DataKeyPath, OperatorData, operators::{Argument, Expr, Reference}}, operators::{OperatorExecutionErrorReason, OperatorExecutionErrorResult, OperatorParsingErrorReason, OperatorPayload, OperatorPriority, OperatorPriorityRank}};

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
    fn execute(&self, data: &mut OperatorData, path: &DataKeyPath) -> OperatorExecutionErrorResult {
        Err(OperatorExecutionErrorReason::Unimplemented)
    }
}

impl OperatorPriority for GrabOperator {
    fn priority(&self) -> OperatorPriorityRank {
        OperatorPriorityRank::AfterFirst
    }
}
