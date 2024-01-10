mod display;

use std::ops::Deref;

pub(crate) mod primary;
pub(crate) use primary::*;

#[cfg(test)]
use super::location::Location;

#[derive(Debug)]
pub(crate) struct Program(pub(crate) Vec<Statement>);

impl Deref for Program {
    type Target = [Statement];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub(crate) enum Statement {
    Expression(Expression),
    Print(Expression),
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Expression {
    Equality(Equality),
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Equality {
    Comparison(Comparison),
    EqualityCheck {
        left: Box<Equality>,
        right: Comparison,
    },
    InequalityCheck {
        left: Box<Equality>,
        right: Comparison,
    },
}

#[cfg(test)]
impl Equality {
    pub(crate) fn string_equality(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Equality::Comparison(Comparison::Term(Term::Factor(Factor::Unary(
            Unary::Primary(Primary::String(StringLiteral::new_string(
                i1.into(),
                Location::default(),
            ))),
        ))));
        let right = Comparison::Term(Term::Factor(Factor::Unary(Unary::Primary(
            Primary::String(StringLiteral::new_string(i2.into(), Location::default())),
        ))));
        Equality::EqualityCheck {
            left: Box::new(left),
            right,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Comparison {
    Term(Term),
    Greater { left: Box<Comparison>, right: Term },
    GreaterEqual { left: Box<Comparison>, right: Term },
    Less { left: Box<Comparison>, right: Term },
    LessEqual { left: Box<Comparison>, right: Term },
}

#[cfg(test)]
impl Comparison {
    pub(crate) fn string_less_equal(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Comparison::Term(Term::Factor(Factor::Unary(Unary::Primary(
            Primary::String(StringLiteral::new_string(i1.into(), Location::default())),
        ))));
        let right = Term::Factor(Factor::Unary(Unary::Primary(Primary::String(
            StringLiteral::new_string(i2.into(), Location::default()),
        ))));
        Comparison::LessEqual {
            left: Box::new(left),
            right,
        }
    }
}

impl From<Comparison> for Expression {
    fn from(comp: Comparison) -> Self {
        Expression::Equality(Equality::Comparison(comp))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Term {
    Factor(Factor),
    Addition { left: Box<Term>, right: Factor },
    Subtraction { left: Box<Term>, right: Factor },
}

#[cfg(test)]
impl Term {
    pub(crate) fn string_addition(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Term::Factor(Factor::Unary(Unary::Primary(Primary::String(
            StringLiteral::new_string(i1.into(), Location::default()),
        ))));
        let right = Factor::Unary(Unary::Primary(Primary::String(StringLiteral::new_string(
            i2.into(),
            Location::default(),
        ))));
        Term::Addition {
            left: Box::new(left),
            right,
        }
    }
}

impl From<Term> for Expression {
    fn from(term: Term) -> Self {
        Expression::Equality(Equality::Comparison(Comparison::Term(term)))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Factor {
    Unary(Unary),
    Multiplication { left: Box<Factor>, right: Unary },
    Division { left: Box<Factor>, right: Unary },
}

#[cfg(test)]
impl Factor {
    pub(crate) fn string_multiplication(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Factor::Unary(Unary::Primary(Primary::String(StringLiteral::new_string(
            i1.into(),
            Location::default(),
        ))));
        let right = Unary::Primary(Primary::String(StringLiteral::new_string(
            i2.into(),
            Location::default(),
        )));
        Factor::Multiplication {
            left: Box::new(left),
            right,
        }
    }
}

impl From<Factor> for Expression {
    fn from(factor: Factor) -> Self {
        Expression::Equality(Equality::Comparison(Comparison::Term(Term::Factor(factor))))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Unary {
    Primary(Primary),
    LogicalNegation(Box<Unary>),
    ArithmNegation(Box<Unary>),
}

#[cfg(test)]
impl Unary {
    pub(crate) fn string_arithm_negation(i: impl Into<String>) -> Self {
        Unary::ArithmNegation(Box::new(Unary::Primary(Primary::String(
            StringLiteral::new_string(i.into(), Location::default()),
        ))))
    }
}

impl From<Unary> for Expression {
    fn from(unary: Unary) -> Self {
        Expression::Equality(Equality::Comparison(Comparison::Term(Term::Factor(
            Factor::Unary(unary),
        ))))
    }
}
