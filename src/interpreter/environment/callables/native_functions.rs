use crate::{
    domain::{grammar::Callee, location::CodeSpan},
    interpreter::error::CallError,
    Environment, Value, ValueType,
};

pub(super) fn current_time_s() -> Value {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs_f64();
    Value::new(ValueType::Number(time), CodeSpan::default())
}

pub(super) fn meaning_of_life(args: Vec<Value>) -> Result<Value, CallError> {
    assert!(args.is_empty(), "already checked that arg num is 0");
    let res = Value::new(ValueType::Number(42.0), CodeSpan::default());
    Ok(res)
}

pub(super) fn is_meaning_of_life(mut args: Vec<Value>) -> Result<Value, CallError> {
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

impl Environment {
    pub(super) fn call_native_func(
        &mut self,
        func: NativeFunction,
        callee: &Callee,
    ) -> Result<Value, CallError> {
        let args = self.interpret_args(callee)?;
        match func {
            NativeFunction::MeaningOfLife => meaning_of_life(args),
            NativeFunction::IsMeaningOfLife => is_meaning_of_life(args),
            NativeFunction::CurrentTimeMs => Ok(current_time_s()),
        }
    }
}

pub(super) enum NativeFunction {
    MeaningOfLife,
    IsMeaningOfLife,
    CurrentTimeMs,
}

impl TryFrom<&str> for NativeFunction {
    type Error = CallError;

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            "meaning_of_life" => Ok(NativeFunction::MeaningOfLife),
            "is_meaning_of_life" => Ok(NativeFunction::IsMeaningOfLife),
            "current_time_s" => Ok(NativeFunction::CurrentTimeMs),
            _ => Err(CallError::unknown_identifier(value)),
        }
    }
}

impl AsRef<str> for NativeFunction {
    fn as_ref(&self) -> &str {
        match self {
            NativeFunction::MeaningOfLife => "meaning_of_life",
            NativeFunction::IsMeaningOfLife => "is_meaning_of_life",
            NativeFunction::CurrentTimeMs => "current_time_s",
        }
    }
}

impl NativeFunction {
    pub(super) fn arg_num(&self) -> usize {
        match self {
            NativeFunction::MeaningOfLife => 0,
            NativeFunction::IsMeaningOfLife => 1,
            NativeFunction::CurrentTimeMs => 0,
        }
    }
}
