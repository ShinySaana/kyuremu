//! Operators declaration and definition happens here.
//! Should ultimately support plugins in its API.

use crate::data::{operators::{Argument, Expr, Reference, StringLiteral}, OperatorData};

enum OperatorParsingError {
    NameDoesNotMatch,
    ArgumentsLengthDoesNotMatch,
    ArgumentsTypesDoNotMatch,
}

trait Operator {
    fn from_expr(expr: &Expr) -> Result<Self, OperatorParsingError> where Self: Sized;
    fn execute(&self, data: &mut OperatorData, path: &[&str]) -> Result<(), ()>;
}

enum OperatorSource {
    Native(NativeOperator),
}

enum NativeOperator {
    Grab,
    Expect,
}

struct GrabOperator {
    reference: Reference
}

impl Operator for GrabOperator {
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

    fn execute(&self, data: &mut OperatorData, path: &[&str]) -> Result<(), ()> {
        Err(())
    }
}

struct ExpectOperator {
    error_msg: StringLiteral
}

impl Operator for ExpectOperator {
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

    fn execute(&self, data: &mut OperatorData, path: &[&str]) -> Result<(), ()> {
        Err(())
    }
}
