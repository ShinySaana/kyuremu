//! Operators declaration and definition happens here.
//! Should ultimately support plugins in its API.

use std::rc::Rc;

use crate::data::{operators::{Argument, Expr, Reference, StringLiteral}, DataKeyPath, RawOperatorData};

pub enum OperatorParsingErrorReason {
    NoneMatched,
    NameDoesNotMatch,
    ArgumentsLengthDoesNotMatch,
    ArgumentsTypesDoNotMatch,
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

// Ok this NEEDS to be smarter I must have had a brain fart somewhere
// smth smth proc macro?
impl NativeOperator {
    pub fn try_parsing_operator(expr: &Expr) -> Result<Operator, OperatorParsingError> {
        let maybe_grab = GrabOperator::try_from(expr);
        match maybe_grab {
            Ok(grab) => { 
                return Ok(Operator {
                    source: OperatorSource::Native(NativeOperator::Grab),
                    payload: Rc::new(grab)
                });
            },
            Err(error) => {
                match error {
                    OperatorParsingErrorReason::NameDoesNotMatch => {},
                    _ => return Err((Some(NativeOperator::Grab), error))
                }
            }
        };

        let maybe_expect: Result<ExpectOperator, OperatorParsingErrorReason> = ExpectOperator::try_from(expr);
        match maybe_expect {
            Ok(expect) => { 
                return Ok(Operator {
                    source: OperatorSource::Native(NativeOperator::Expect),
                    payload: Rc::new(expect)
                });
            },
            Err(error) => {
                match error {
                    OperatorParsingErrorReason::NameDoesNotMatch => {},
                    _ => return Err((Some(NativeOperator::Expect), error))
                }
            }
        };

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
