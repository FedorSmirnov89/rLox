//! Module for the logic of how things are called

use anyhow::Result;

use crate::{
    domain::{grammar::Callee, location::CodeSpan},
    interpreter::{error::CallError, expressions::InterpretedExpression},
    Environment, Value, ValueType,
};

enum NativeFunction {
    MeaningOfLife,
    IsMeaningOfLife,
}

impl TryFrom<&str> for NativeFunction {
    type Error = CallError;

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            "meaning_of_life" => Ok(NativeFunction::MeaningOfLife),
            "is_meaning_of_life" => Ok(NativeFunction::IsMeaningOfLife),
            _ => Err(CallError::unknown_identifier(value)),
        }
    }
}

impl AsRef<str> for NativeFunction {
    fn as_ref(&self) -> &str {
        match self {
            NativeFunction::MeaningOfLife => "meaning_of_life",
            NativeFunction::IsMeaningOfLife => "is_meaning_of_life",
        }
    }
}

impl NativeFunction {
    fn arg_num(&self) -> usize {
        match self {
            NativeFunction::MeaningOfLife => 0,
            NativeFunction::IsMeaningOfLife => 1,
        }
    }
}

impl Environment {
    pub(crate) fn call(&self, callee: &Callee) -> Result<Value, CallError> {
        let checked_name = check_name(&callee)?;
        let checked_arg_num = check_argument_num(checked_name, callee)?;
        self.call_native_func(checked_arg_num, callee)
    }

    fn call_native_func(&self, func: NativeFunction, callee: &Callee) -> Result<Value, CallError> {
        let args = self.interpret_args(callee)?;
        match func {
            NativeFunction::MeaningOfLife => meaning_of_life_native(args),
            NativeFunction::IsMeaningOfLife => is_meaning_of_life_native(args),
        }
    }

    fn interpret_args(&self, callee: &Callee) -> Result<Vec<Value>, CallError> {
        let args = callee.arguments();
        let mut interpreted_args = vec![];
        for (pos, arg) in args.iter().enumerate() {
            let arg_val = match arg.interpret_expression(self) {
                Ok(val) => val,
                Err(interpret_err) => {
                    let err_msg = format!("{interpret_err:?}");
                    return Err(CallError::invalid_arg(callee.identifier(), pos, err_msg));
                }
            };
            interpreted_args.push(arg_val);
        }
        Ok(interpreted_args)
    }
}

fn check_argument_num(
    checked_name: NativeFunction,
    callee: &Callee,
) -> Result<NativeFunction, CallError> {
    let expected = checked_name.arg_num();
    let found = callee.arg_num();
    if found == expected {
        Ok(checked_name)
    } else {
        Err(CallError::wrong_arg_num(
            checked_name.as_ref(),
            expected,
            found,
        ))
    }
}

fn check_name(callee: &Callee) -> Result<NativeFunction, CallError> {
    callee.identifier().try_into()
}

fn meaning_of_life_native(args: Vec<Value>) -> Result<Value, CallError> {
    assert!(args.is_empty(), "already checked that arg num is 0");
    let res = Value::new(ValueType::Number(42.0), CodeSpan::default());
    Ok(res)
}

fn is_meaning_of_life_native(mut args: Vec<Value>) -> Result<Value, CallError> {
    assert!(args.len() == 1, "already checked that arg num is 1");
    let arg = args.pop().unwrap();

    let ValueType::Number(input) = arg.v_type else {
        return Err(CallError::wrong_arg_type(
            "is_meaning_of_life",
            0,
            ValueType::Number(0.0),
            arg.v_type,
        ));
    };

    if input == 42.0 {
        Ok(Value::new(ValueType::Boolean(true), arg.span()))
    } else {
        Ok(Value::new(ValueType::Boolean(false), arg.span()))
    }
}
