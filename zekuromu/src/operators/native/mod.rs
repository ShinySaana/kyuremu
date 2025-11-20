use std::rc::Rc;

use crate::{data::operators::Expr, operators::{Operator, OperatorParsingError, OperatorParsingErrorReason, OperatorPayload, OperatorPriority, OperatorSource, native::{expect::ExpectOperator, grab::GrabOperator, param::ParamOperator}}};

pub mod expect;
pub mod grab;
pub mod param;

#[derive(Debug, Clone)]
pub enum NativeOperator {
    Expect,
    Grab,
    Param,
}

fn try_parse_native<'a, T>(expr: &'a Expr, kind: NativeOperator) -> Option<Result<Operator, OperatorParsingError>>
where T: TryFrom<&'a Expr, Error = OperatorParsingErrorReason> + OperatorPayload + OperatorPriority + 'static
{
    let maybe_op: Result<Rc<T>, OperatorParsingErrorReason> = expr.try_into().map(|op| Rc::new(op));

    match maybe_op {
        Ok(op) => {
            let priority = op.priority();
            Some(Ok( Operator {
            source: OperatorSource::Native(kind),
            payload: op,
            priority: priority,
        }))
        },
        Err(error) => {
            match error {
                OperatorParsingErrorReason::NameDoesNotMatch => None,
                _ => Some(Err((
                        Some(kind),
                        error
                )))
            }
        }
    }
}

// Better but still should be a macro at some point
impl NativeOperator {
    pub fn try_parsing_operator(expr: &Expr) -> Result<Operator, OperatorParsingError> {
        if let Some(operator) = try_parse_native::<ExpectOperator>(expr, NativeOperator::Expect) {
            return operator;
        }

        if let Some(operator) = try_parse_native::<ParamOperator>(expr, NativeOperator::Param) {
            return operator;
        }

        if let Some(operator) = try_parse_native::<GrabOperator>(expr, NativeOperator::Grab) {
            return operator;
        }

        Err((None, OperatorParsingErrorReason::NoneMatched))
    }
}

