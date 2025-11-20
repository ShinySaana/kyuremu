//! Operators to enrich, prune, reference, or allow more complex array operations.
//! Heavily inspired by [spruce](https://github.com/geofffranks/spruce).
//! Operators declaration and definition happens here.
//! Should ultimately support plugins in its API.

use std::rc::Rc;

use crate::{data::{DataKeyPath, OperatorData}, operators::native::NativeOperator};

pub mod native;

#[derive(Debug, Clone)]
pub enum OperatorParsingErrorReason {
    NoneMatched,
    NameDoesNotMatch,
    ArgumentsLengthDoesNotMatch,
    ArgumentsTypesDoNotMatch,
    Unknown,
}

pub type OperatorParsingError = (Option<NativeOperator>, OperatorParsingErrorReason);

#[derive(Debug, Clone)]
pub enum OperatorExecutionErrorReason {
    Unimplemented,
    ReferenceUnavailable,
    OtherError(String),
}

pub type OperatorExecutionErrorResult = Result<(), OperatorExecutionErrorReason>;

pub trait OperatorPayload : std::fmt::Debug {
    fn execute(&self, data: &mut OperatorData, path: &DataKeyPath) -> OperatorExecutionErrorResult;
}

#[derive(Debug, Clone)]
enum OperatorSource {
    Native(NativeOperator),
}

#[derive(Debug, Clone, Copy)]
pub enum OperatorPriorityRank {
    /// Mostly for `param`.
    First,
    /// Mostly for setup operators.
    AfterFirst,
    /// For most operators.
    Middle,
    /// Mostly for array operators.
    BeforeLast,
    /// Mostly for `expect`.
    Last,
}

pub trait OperatorPriority {
    fn priority(&self) -> OperatorPriorityRank;
}

#[derive(Debug, Clone)]
pub struct Operator {
    source: OperatorSource,
    payload: Rc<dyn OperatorPayload>,
    priority: OperatorPriorityRank,
}

impl Operator {
    pub fn execute(&self, data: &mut OperatorData, path: &DataKeyPath) -> OperatorExecutionErrorResult {
        self.payload.execute(data, path)
    }
}
