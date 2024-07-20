//! Module for the logic of how things are called

use anyhow::Result;

use crate::{
    domain::grammar::{Callee, FunDeclaration},
    interpreter::{error::CallError, expressions::InterpretedExpression, Outcome},
    Environment, Value,
};

use self::native_functions::NativeFunction;

mod native_functions;

impl Environment {
    pub(crate) fn call(&mut self, callee: &Callee) -> Result<Value, CallError> {
        let checked_name = self.check_name(&callee)?;
        let checked_arg_num = check_argument_num(checked_name, callee)?;
        match checked_arg_num {
            Function::Native(native) => self.call_native_func(native, callee),
            Function::UserDefined(fun) => {
                let args = self.interpret_args(callee)?;
                self.call_user_function(fun, args)
            }
        }
    }

    fn interpret_args(&mut self, callee: &Callee) -> Result<Vec<Value>, CallError> {
        let args = callee.arguments();
        let mut interpreted_args = vec![];
        for (pos, arg) in args.iter().enumerate() {
            let arg_outcome = match arg.interpret_expression(self) {
                Ok(val) => val,
                Err(interpret_err) => {
                    let err_msg = format!("{interpret_err:?}");
                    return Err(CallError::invalid_arg(callee.identifier(), pos, err_msg));
                }
            };

            // in case of an argument expression, we would for now treat a return as error
            let arg_val = match arg_outcome {
                Outcome::Value(val) => val,
                Outcome::Return(_) | Outcome::Void => {
                    let err_msg = "expression used as argument did not return a value";
                    return Err(CallError::invalid_arg(callee.identifier(), pos, err_msg));
                }
            };
            interpreted_args.push(arg_val);
        }
        Ok(interpreted_args)
    }

    fn check_name(&self, callee: &Callee) -> Result<Function, CallError> {
        let name = callee.identifier();
        if let Some(user_fun) = self.scope().get_fun(name) {
            Ok(Function::UserDefined(user_fun.clone()))
        } else {
            let native_fun = callee.identifier().try_into()?;
            Ok(Function::Native(native_fun))
        }
    }

    fn call_user_function(
        &mut self,
        fun: FunDeclaration,
        args: Vec<Value>,
    ) -> Result<Value, CallError> {
        self.new_inner_scope();
        for (iden, val) in fun.arguments.iter().zip(args) {
            self.scope_mut().declare_var(iden.clone());
            self.scope_mut()
                .set_var_value(iden, val)
                .map_err(|_err| CallError::ParameterProblem)?;
        }
        let block = &fun.body;
        let block_result = block
            .interpret_expression(self)
            .map_err(|inter_err| CallError::InterpreterProblem(Box::new(inter_err)))?;
        self.teardown_inner_scope();

        // in case of a function block, an early return is actually fine and should be returned
        let block_val = match block_result {
            Outcome::Return(val) => val,
            Outcome::Value(val) => val,
            Outcome::Void => {
                unimplemented!(
                    "tbh, I am not sure what a void return of a functions means right now"
                );
            }
        };

        Ok(block_val)
    }
}

fn check_argument_num(checked_name: Function, callee: &Callee) -> Result<Function, CallError> {
    let expected = checked_name.arg_num();
    let found = callee.arg_num();
    if found == expected {
        Ok(checked_name)
    } else {
        Err(CallError::wrong_arg_num(
            checked_name.name(),
            expected,
            found,
        ))
    }
}

enum Function {
    Native(NativeFunction),
    UserDefined(FunDeclaration),
}

impl Function {
    fn arg_num(&self) -> usize {
        match self {
            Function::Native(native) => native.arg_num(),
            Function::UserDefined(user) => user.arguments.len(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Function::Native(native) => native.as_ref(),
            Function::UserDefined(user) => &user.name,
        }
    }
}
