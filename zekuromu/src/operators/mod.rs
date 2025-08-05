//! Operators declaration and definition happens here.
//! Should ultimately support plugins in its API.

use crate::data::{operators::{Argument, Expr, Reference, StringLiteral}, DataKeyPath, RawOperatorData};

pub enum OperatorParsingError {
    NoneMatched,
    NameDoesNotMatch,
    ArgumentsLengthDoesNotMatch,
    ArgumentsTypesDoNotMatch,
}

pub trait OperatorPayload {
    fn from_expr(expr: &Expr) -> Result<Self, OperatorParsingError> where Self: Sized;
    fn execute(&self, data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String>;
} 

pub struct Operator {
    source: OperatorSource,
    payload: Box<dyn OperatorPayload>
}

impl Operator {

}

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
    fn try_parsing_operator(expr: &Expr) -> Result<Operator, (Option<NativeOperator>, OperatorParsingError)> where Self: Sized {
        let maybe_grab = GrabOperator::from_expr(expr);
        match maybe_grab {
            Ok(grab) => { 
                return Ok(Operator {
                    source: OperatorSource::Native(NativeOperator::Grab),
                    payload: Box::new(grab)
                });
            },
            Err(error) => {
                match error {
                    OperatorParsingError::NameDoesNotMatch => {},
                    _ => return Err((Some(NativeOperator::Grab), error))
                }
            }
        };

        let maybe_expect = ExpectOperator::from_expr(expr);
        match maybe_expect {
            Ok(expect) => { 
                return Ok(Operator {
                    source: OperatorSource::Native(NativeOperator::Expect),
                    payload: Box::new(expect)
                });
            },
            Err(error) => {
                match error {
                    OperatorParsingError::NameDoesNotMatch => {},
                    _ => return Err((Some(NativeOperator::Expect), error))
                }
            }
        };

        Err((None, OperatorParsingError::NoneMatched))
    }
}

#[derive(Debug, Clone)]
pub struct GrabOperator {
    reference: Reference
}

impl OperatorPayload for GrabOperator {
    fn from_expr(expr: &Expr) -> Result<Self, OperatorParsingError> where Self: Sized {
        if &expr.name.0 != "grab" {
            return Err(OperatorParsingError::NameDoesNotMatch)
        }

        if expr.arguments.len() != 1 {
            return Err(OperatorParsingError::ArgumentsLengthDoesNotMatch)
        }

        let first_arg = expr.arguments.get(1).unwrap();
        match first_arg {
            Argument::Reference(inner) => Ok(GrabOperator { reference: inner.clone() }),
            _ => Err(OperatorParsingError::ArgumentsTypesDoNotMatch)
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
    fn from_expr(expr: &Expr) -> Result<Self, OperatorParsingError> where Self: Sized {
        if &expr.name.0 != "expect" {
            return Err(OperatorParsingError::NameDoesNotMatch)
        }

        if expr.arguments.len() != 1 {
            return Err(OperatorParsingError::ArgumentsLengthDoesNotMatch)
        }

        let first_arg = expr.arguments.get(1).unwrap();
        match first_arg {
            Argument::StringLiteral(inner) => Ok(ExpectOperator { error_msg: inner.clone() }),
            _ => Err(OperatorParsingError::ArgumentsTypesDoNotMatch)
        }
    }

    fn execute(&self, _data: &mut RawOperatorData, path: &DataKeyPath) -> Result<(), String> {
        Err(format!("At path '{}', expected a value. Provided error message: '{}'", path, self.error_msg.0))
    }
}
