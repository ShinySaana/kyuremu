//! Operators declaration and definition happens here.
//! Should ultimately support plugins in its API.

use std::rc::Rc;

use crate::data::{operators::{Argument, Expr, Reference, StringLiteral}, DataKeyPath, RawOperatorData};

pub enum OperatorParsingErrorReason {
    NoneMatched,
    NameDoesNotMatch,
    ArgumentsLengthDoesNotMatch,
    ArgumentsTypesDoNotMatch,
    Unknown,
}

pub type OperatorParsingError = (Option<NativeOperator>, OperatorParsingErrorReason);

pub trait OperatorPayload : std::fmt::Debug {
    fn execute(&self, data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct Operator {

    source: OperatorSource,
    payload: Rc<dyn OperatorPayload>
}

impl Operator {

}

#[derive(Debug, Clone)]
enum OperatorSource {
    Native(NativeOperator),
}

#[derive(Debug, Clone)]
pub enum NativeOperator {
    Grab,
    Expect,
}

fn try_parse_native<'a, T>(expr: &'a Expr, kind: NativeOperator) -> Option<Result<Operator, OperatorParsingError>>
where T: TryFrom<&'a Expr, Error = OperatorParsingErrorReason> + OperatorPayload + 'static
{
    let maybe_op: Result<Rc<T>, OperatorParsingErrorReason> = expr.try_into().map(|op| Rc::new(op));

    match maybe_op {
        Ok(op) => Some(Ok( Operator {
            source: OperatorSource::Native(kind),
            payload: op
        })),
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
        if let Some(operator) = try_parse_native::<GrabOperator>(expr, NativeOperator::Grab) {
            return operator;
        }

        if let Some(operator) = try_parse_native::<ExpectOperator>(expr, NativeOperator::Expect) {
            return operator;
        }

        Err((None, OperatorParsingErrorReason::NoneMatched))
    }
}

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

        let first_arg = value.arguments.get(1).unwrap();
        match first_arg {
            Argument::Reference(inner) => Ok(GrabOperator { reference: inner.clone() }),
            _ => Err(OperatorParsingErrorReason::ArgumentsTypesDoNotMatch)
        }    }
}


impl OperatorPayload for GrabOperator {
    fn execute(&self, data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String> {
        Err("Unimplemented".into())
    }
}

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

        let first_arg = value.arguments.get(1).unwrap();
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
