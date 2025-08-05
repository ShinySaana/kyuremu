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
    fn from_expr(expr: &Expr) -> Result<Self, OperatorParsingErrorReason> where Self: Sized;
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
    pub fn try_parsing_operator(expr: &Expr) -> Result<Operator, OperatorParsingError> where Self: Sized {
        let maybe_grab = GrabOperator::from_expr(expr);
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

        let maybe_expect = ExpectOperator::from_expr(expr);
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

impl OperatorPayload for GrabOperator {
    fn from_expr(expr: &Expr) -> Result<Self, OperatorParsingErrorReason> where Self: Sized {
        if &expr.name.0 != "grab" {
            return Err(OperatorParsingErrorReason::NameDoesNotMatch)
        }

        if expr.arguments.len() != 1 {
            return Err(OperatorParsingErrorReason::ArgumentsLengthDoesNotMatch)
        }

        let first_arg = expr.arguments.get(1).unwrap();
        match first_arg {
            Argument::Reference(inner) => Ok(GrabOperator { reference: inner.clone() }),
            _ => Err(OperatorParsingErrorReason::ArgumentsTypesDoNotMatch)
        }
    }

    fn execute(&self, data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String> {
        Err("Unimplemented".into())
    }
}

#[derive(Debug, Clone)]
pub struct ExpectOperator {
    error_msg: StringLiteral
}

impl OperatorPayload for ExpectOperator {
    fn from_expr(expr: &Expr) -> Result<Self, OperatorParsingErrorReason> where Self: Sized {
        if &expr.name.0 != "expect" {
            return Err(OperatorParsingErrorReason::NameDoesNotMatch)
        }

        if expr.arguments.len() != 1 {
            return Err(OperatorParsingErrorReason::ArgumentsLengthDoesNotMatch)
        }

        let first_arg = expr.arguments.get(1).unwrap();
        match first_arg {
            Argument::StringLiteral(inner) => Ok(ExpectOperator { error_msg: inner.clone() }),
            _ => Err(OperatorParsingErrorReason::ArgumentsTypesDoNotMatch)
        }
    }

    fn execute(&self, _data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String> {
        Err(format!("At path '{}', expected a value. Provided error message: '{}'", path, self.error_msg.0))
    }
}
