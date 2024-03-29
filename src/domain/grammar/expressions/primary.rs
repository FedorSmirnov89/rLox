use anyhow::{bail, Result};

use crate::domain::{
    location::{CodeSpan, Location},
    scanning::{Token, TokenType},
};

use super::Expression;

#[cfg(test)]
use super::{Comparison, Equality, Factor, Term, Unary};

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Primary {
    Number(NumLiteral),
    String(StringLiteral),
    Identifier(StringLiteral),
    True(CodeSpan),
    False(CodeSpan),
    Nil(CodeSpan),
    GroupedExpression(Box<Expression>),
}

impl Primary {
    pub(crate) fn true_literal(start: Location) -> Self {
        let end = start.shifted(4);
        Primary::True(CodeSpan { start, end })
    }

    pub(crate) fn false_literal(start: Location) -> Self {
        let end = start.shifted(5);
        Primary::False(CodeSpan { start, end })
    }

    pub(crate) fn nil_literal(start: Location) -> Self {
        let end = start.shifted(3);
        Primary::Nil(CodeSpan { start, end })
    }
}

#[cfg(test)]
impl From<StringLiteral> for Primary {
    fn from(value: StringLiteral) -> Self {
        Primary::String(value)
    }
}

#[cfg(test)]
impl From<Primary> for Unary {
    fn from(value: Primary) -> Self {
        Unary::Primary(value)
    }
}

#[cfg(test)]
impl From<Primary> for Expression {
    fn from(value: Primary) -> Self {
        let unary: Unary = value.into();
        unary.into()
    }
}

#[cfg(test)]
impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        let num_lit: NumLiteral = NumLiteral::new(value, Location::default());
        Equality::Comparison(Comparison::Term(Term::Factor(Factor::Unary(
            Unary::Primary(Primary::Number(num_lit)),
        ))))
        .into()
    }
}

#[cfg(test)]
impl Primary {
    pub(crate) fn grouped_expr(expr: Expression) -> Expression {
        let equality = Equality::Comparison(Comparison::Term(Term::Factor(Factor::Unary(
            Unary::Primary(Primary::GroupedExpression(Box::new(expr))),
        ))));
        equality.into()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct StringLiteral {
    pub(crate) value: String,
    pub(crate) span: CodeSpan,
}

impl StringLiteral {
    fn new(value: String, span: CodeSpan) -> Self {
        Self { value, span }
    }

    pub(crate) fn new_string(value: impl Into<String>, start: Location) -> Self {
        let value = value.into();
        let end = start.shifted(value.len() + 2);
        let span = CodeSpan { start, end };
        Self::new(value, span)
    }

    pub(crate) fn new_identifier(value: impl Into<String>, start: Location) -> Self {
        let value = value.into();
        let end = start.shifted(value.len());
        let span = CodeSpan { start, end };
        Self::new(value, span)
    }

    pub(crate) fn identifier_from_token(token: &Token) -> Result<Self> {
        let TokenType::Identifier(iden) = token.t_type() else {
            bail!("current is not an identifier")
        };
        let start = token.location();
        Ok(Self::new_identifier(iden, start))
    }
}

impl AsRef<str> for StringLiteral {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) struct NumLiteral {
    pub(crate) value: f64,
    pub(crate) span: CodeSpan,
}

impl NumLiteral {
    pub(crate) fn new(value: f64, start: Location) -> Self {
        let len = value.to_string().len();
        let end = start.shifted(len);
        let span = CodeSpan { start, end };
        Self { value, span }
    }
}

impl AsRef<f64> for NumLiteral {
    fn as_ref(&self) -> &f64 {
        &self.value
    }
}

impl Eq for NumLiteral {}
