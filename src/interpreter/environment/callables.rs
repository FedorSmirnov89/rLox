//! Module for the logic of how things are called

use anyhow::{bail, Result};

use crate::{
    domain::{
        grammar::{Callee, CalleeIdentifier, Expression},
        location::CodeSpan,
    },
    interpreter::expressions::InterpretedExpression,
    Environment, Value, ValueType,
};

enum NativeFunction {
    MeaningOfLife,
}

impl Environment {
    pub(crate) fn call(&self, callee: &Callee) -> Result<Value> {
        let checked_name = check_name(&callee)?;
        let checked_arg_num = check_argument_num(checked_name, callee)?;
        self.call_native_func(checked_arg_num, callee)
    }

    fn call_native_func(&self, func: NativeFunction, callee: &Callee) -> Result<Value> {
        let args = self.interpret_args(&callee.arguments())?;
        match func {
            NativeFunction::MeaningOfLife => meaning_of_life_native(args),
        }
    }

    fn interpret_args(&self, args: &[Expression]) -> Result<Vec<Value>> {
        let mut interpreted_args = vec![];
        for arg in args {
            interpreted_args.push(arg.interpret_expression(self)?);
        }
        Ok(interpreted_args)
    }
}

fn check_argument_num(checked_name: NativeFunction, callee: &Callee) -> Result<NativeFunction> {
    match checked_name {
        NativeFunction::MeaningOfLife => match callee.arg_num() {
            0 => Ok(checked_name),
            _ => bail!("meaning of life does not take args"),
        },
    }
}

fn check_name(callee: &Callee) -> Result<NativeFunction> {
    match callee.identifier() {
        CalleeIdentifier::Identifier(s) if s == "meaning_of_life" => {
            Ok(NativeFunction::MeaningOfLife)
        }
        _ => bail!(
            "Unknown function identifier: {iden}",
            iden = callee.identifier()
        ),
    }
}

fn meaning_of_life_native(args: Vec<Value>) -> Result<Value> {
    if !args.is_empty() {
        bail!("meaning of life does not take arguments");
    }
    let res = Value::new(ValueType::Number(42.0), CodeSpan::default());
    Ok(res)
}
