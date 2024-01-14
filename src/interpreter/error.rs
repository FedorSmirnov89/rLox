use std::fmt::Display;

use crate::domain::location::CodeSpan;

#[derive(Debug)]
pub enum InterpreterError {
    BinaryOperatorError(BinaryOperatorError),
    UnaryOperatorError(UnaryOperatorError),
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

    pub fn unary_operator(msg: String, span_operator: CodeSpan, span_operand: CodeSpan) -> Self {
        Self::UnaryOperatorError(UnaryOperatorError {
            msg,
            span_operator,
            span_operand,
        })
    }

    pub fn msg(self, src_str: &str) -> String {
        match self {
            Self::BinaryOperatorError(e) => e.msg(src_str),
            Self::UnaryOperatorError(e) => e.msg(src_str),
        }
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
        dbg!(&self.span_left.start.pos);
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
