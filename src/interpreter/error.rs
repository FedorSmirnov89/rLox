use std::fmt::Display;

use crate::{
    domain::{grammar::StringLiteral, location::CodeSpan},
    Value, ValueType,
};

#[derive(Debug)]
pub enum InterpreterError {
    BinaryOperatorError(BinaryOperatorError),
    UnaryOperatorError(UnaryOperatorError),
    IdentifierNotDefinedError(IdentifierNotDefinedError),
    FunctionAlreadyDeclared(String),
    TypeError(TypeError),
    CallError(CallError),
}

#[derive(Debug)]
pub(crate) enum CallError {
    UnknownIdentifier(String),
    WrongArgNum {
        iden: String,
        expected: usize,
        found: usize,
    },
    WrongArgType {
        iden: String,
        pos: usize,
        expected: ValueType,
        found: ValueType,
    },
    InvalidArg {
        iden: String,
        pos: usize,
        msg: String,
    },
    ParameterProblem,
    InterpreterProblem(Box<InterpreterError>),
}

impl CallError {
    pub(crate) fn invalid_arg(iden: impl Into<String>, pos: usize, msg: impl Into<String>) -> Self {
        Self::InvalidArg {
            iden: iden.into(),
            pos,
            msg: msg.into(),
        }
    }

    pub(crate) fn wrong_arg_type(
        iden: impl Into<String>,
        pos: usize,
        expected: ValueType,
        found: ValueType,
    ) -> Self {
        Self::WrongArgType {
            iden: iden.into(),
            pos,
            expected,
            found,
        }
    }

    pub(crate) fn wrong_arg_num(iden: impl Into<String>, expected: usize, found: usize) -> Self {
        Self::WrongArgNum {
            iden: iden.into(),
            expected,
            found,
        }
    }

    pub(crate) fn unknown_identifier(name: impl Into<String>) -> Self {
        Self::UnknownIdentifier(name.into())
    }

    fn msg(&self) -> String {
        match self {
            CallError::UnknownIdentifier(unknown) => format!("no callable called '{unknown}' is known"),
            CallError::WrongArgNum {
                expected,
                found,
                iden,
            } => format!("wrong number of arguments for callable '{iden}': expected {expected}, found {found} arguments"),
            CallError::WrongArgType {
                iden,
                pos,
                expected,
                found,
            } => format!("wrong argumet type for the {pos}-th argument of callable '{iden}': expected {expected}, found {found}"),
            CallError::InvalidArg {iden, pos, msg } => format!("failed to parse the {pos}-th argument of the callee called '{iden}'. Interpreter error message: {msg}"),
            CallError::ParameterProblem => "problem with parameters of user function".to_owned(),
            CallError::InterpreterProblem(err) => format!("interpreter error: {err}"),
        }
    }
}

impl InterpreterError {
    pub fn binary_operator(
        msg: String,
        span_operator: CodeSpan,
        span_left: CodeSpan,
        span_right: CodeSpan,
    ) -> Self {
        Self::BinaryOperatorError(BinaryOperatorError {
            msg,
            span_operator,
            span_left,
            span_right,
        })
    }

    pub fn type_error(expected: ValueType, val: Value, context: &'static str) -> Self {
        let expected = expected.variant_name();
        let actual = val.v_type.variant_name();
        let span = val.span();
        Self::TypeError(TypeError {
            expected,
            actual,
            span,
            context,
        })
    }

    pub fn unary_operator(msg: String, span_operator: CodeSpan, span_operand: CodeSpan) -> Self {
        Self::UnaryOperatorError(UnaryOperatorError {
            msg,
            span_operator,
            span_operand,
        })
    }

    pub fn identifier_not_defined(iden: StringLiteral) -> Self {
        Self::IdentifierNotDefinedError(IdentifierNotDefinedError { iden })
    }

    pub fn msg(self, src_str: &str) -> String {
        match self {
            Self::BinaryOperatorError(e) => e.msg(src_str),
            Self::UnaryOperatorError(e) => e.msg(src_str),
            Self::IdentifierNotDefinedError(e) => e.msg(),
            Self::TypeError(e) => e.msg(),
            Self::CallError(e) => e.msg(),
            Self::FunctionAlreadyDeclared(name) => format!("function '{name}' already declared"),
        }
    }

    pub fn unwrap_bool(val: Value, context: &'static str) -> Result<bool, Self> {
        match val.v_type {
            ValueType::Boolean(b) => Ok(b),
            _ => Err(Self::type_error(ValueType::Boolean(true), val, context)),
        }
    }
}

#[derive(Debug)]
pub struct TypeError {
    pub expected: &'static str,
    pub actual: &'static str,
    pub context: &'static str,
    pub span: CodeSpan,
}

impl TypeError {
    fn msg(self) -> String {
        format!(
            "type error in line {line} - expected: '{expected}'; found: '{actual}'; context: {context}",
            expected = self.expected,
            actual = self.actual,
            context = self.context,
            line = self.span.start.line
        )
    }
}

#[derive(Debug)]
pub struct IdentifierNotDefinedError {
    pub iden: StringLiteral,
}

impl IdentifierNotDefinedError {
    fn msg(self) -> String {
        format!(
            "identifier '{iden}' (used in line {l}) not defined",
            iden = self.iden,
            l = self.iden.span.start.line
        )
    }
}

#[derive(Debug)]
pub struct BinaryOperatorError {
    pub msg: String,
    pub span_operator: CodeSpan,
    pub span_left: CodeSpan,
    pub span_right: CodeSpan,
}

impl BinaryOperatorError {
    fn msg(self, src_str: &str) -> String {
        let mut msg = self.msg;
        msg.push_str("\n");

        msg.push_str("Left operand source:\n");
        let left_str = &src_str[self.span_left.start.pos..self.span_left.end.pos];
        msg.push_str(left_str);
        msg.push_str("\n");

        msg.push_str("operator source:\n");
        let oper_str = &src_str[self.span_operator.start.pos..self.span_operator.end.pos];
        msg.push_str(oper_str);
        msg.push_str("\n");

        msg.push_str("right operand source:\n");
        let right_str = &src_str[self.span_right.start.pos..self.span_right.end.pos];
        msg.push_str(right_str);

        msg
    }
}

#[derive(Debug)]
pub struct UnaryOperatorError {
    pub msg: String,
    pub span_operator: CodeSpan,
    pub span_operand: CodeSpan,
}

impl UnaryOperatorError {
    fn msg(self, src_str: &str) -> String {
        let mut msg = self.msg;
        msg.push_str("\n");

        msg.push_str("operator source:\n");
        let oper_str = &src_str[self.span_operator.start.pos..self.span_operator.end.pos];
        msg.push_str(oper_str);

        msg.push_str("operand source:\n");
        let operand_str = &src_str[self.span_operand.start.pos..self.span_operand.end.pos];
        msg.push_str(operand_str);

        msg
    }
}

impl Display for InterpreterError {
    // TODO: Should have a proper display for different variants
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{message}", message = "interpreter_error")
    }
}

impl std::error::Error for InterpreterError {}

#[macro_export]
macro_rules! operator_error {
    // binary operators
    ($left:ident, $right:ident, $oper_name: expr) => {
        let msg = format!(
            "operator {oper} not defined for types {left} and {right}",
            oper = $oper_name,
            left = $left.v_type,
            right = $right.v_type,
        );

        let err = InterpreterError::binary_operator(
            msg,
            CodeSpan::in_between($left.span(), $right.span()),
            $left.span(),
            $right.span(),
        );
        return Err(err);
    };

    // unary operators
    ($val:ident, $oper_name: expr) => {
        let msg = format!(
            "operator {oper} not defined for type {val}",
            oper = $oper_name,
            val = $val.v_type,
        );

        let err = InterpreterError::unary_operator(msg, $val.span(), $val.span());
        return Err(err);
    };
}
