//! Operators to enrich, prune, reference, or allow more complex array operations.
//! Heavily inspired by [spruce](https://github.com/geofffranks/spruce).
//! Operators declaration and definition happens here.
//! Should ultimately support plugins in its API.

use std::rc::Rc;

use crate::{data::{DataKeyPath, RawOperatorData}, operators::native::NativeOperator};

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


