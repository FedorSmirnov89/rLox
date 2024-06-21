//! Module for the logic of how things are called

use anyhow::Result;

use crate::{
    domain::grammar::Callee,
    interpreter::{error::CallError, expressions::InterpretedExpression},
    Environment, Value,
};

use self::native_functions::NativeFunction;

mod native_functions;

impl Environment {
    pub(crate) fn call(&mut self, callee: &Callee) -> Result<Value, CallError> {
        let checked_name = check_name(&callee)?;
        let checked_arg_num = check_argument_num(checked_name, callee)?;
        self.call_native_func(checked_arg_num, callee)
    }

    fn interpret_args(&mut self, callee: &Callee) -> Result<Vec<Value>, CallError> {
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
