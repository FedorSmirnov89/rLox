mod primary;

pub(crate) use primary::*;

#[cfg(test)]
use crate::domain::location::Location;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum Expression {
    LogicOr(LogicOr),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum LogicOr {
    LogicAnd(LogicAnd),
    Or { left: Box<LogicOr>, right: LogicAnd },
}

impl From<LogicOr> for Expression {
    fn from(value: LogicOr) -> Self {
        Expression::LogicOr(value)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum LogicAnd {
    Equality(Equality),
    And {
        left: Box<LogicAnd>,
        right: Equality,
    },
}

impl From<LogicAnd> for LogicOr {
    fn from(value: LogicAnd) -> Self {
        LogicOr::LogicAnd(value)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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

impl From<Equality> for LogicAnd {
    fn from(value: Equality) -> Self {
        LogicAnd::Equality(value)
    }
}

impl From<Equality> for Expression {
    fn from(value: Equality) -> Self {
        let l_and: LogicAnd = value.into();
        let l_or: LogicOr = l_and.into();
        l_or.into()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
        Expression::LogicOr(LogicOr::LogicAnd(LogicAnd::Equality(Equality::Comparison(
            comp,
        ))))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
        let comparison = Comparison::Term(term);
        comparison.into()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
        let term = Term::Factor(factor);
        term.into()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
        let factor = Factor::Unary(unary);
        factor.into()
    }
}
