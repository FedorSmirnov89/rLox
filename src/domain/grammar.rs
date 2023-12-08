mod display;

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

impl Equality {
    pub(crate) fn string_equality(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Equality::Comparison(Comparison::Term(Term::Factor(Factor::Unary(
            Unary::Primary(Primary::String(i1.into().into())),
        ))));
        let right = Comparison::Term(Term::Factor(Factor::Unary(Unary::Primary(
            Primary::String(i2.into().into()),
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

impl Comparison {
    pub(crate) fn string_less_equal(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Comparison::Term(Term::Factor(Factor::Unary(Unary::Primary(
            Primary::String(i1.into().into()),
        ))));
        let right = Term::Factor(Factor::Unary(Unary::Primary(Primary::String(
            i2.into().into(),
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

impl Term {
    pub(crate) fn string_addition(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Term::Factor(Factor::Unary(Unary::Primary(Primary::String(
            i1.into().into(),
        ))));
        let right = Factor::Unary(Unary::Primary(Primary::String(i2.into().into())));
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

impl Factor {
    pub(crate) fn string_multiplication(i1: impl Into<String>, i2: impl Into<String>) -> Self {
        let left = Factor::Unary(Unary::Primary(Primary::String(i1.into().into())));
        let right = Unary::Primary(Primary::String(i2.into().into()));
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

impl Unary {
    pub(crate) fn string_arithm_negation(i: impl Into<String>) -> Self {
        Unary::ArithmNegation(Box::new(Unary::Primary(Primary::String(i.into().into()))))
    }
}

impl From<Unary> for Expression {
    fn from(unary: Unary) -> Self {
        Expression::Equality(Equality::Comparison(Comparison::Term(Term::Factor(
            Factor::Unary(unary),
        ))))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Primary {
    Number(NumLiteral),
    String(StringLiteral),
    Identifier(StringLiteral),
    True,
    False,
    Nil,
    GroupedExpression(Box<Expression>),
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        let num_lit: NumLiteral = value.into();
        Expression::Equality(Equality::Comparison(Comparison::Term(Term::Factor(
            Factor::Unary(Unary::Primary(Primary::Number(num_lit))),
        ))))
    }
}

impl Primary {
    pub(crate) fn grouped_expr(expr: Expression) -> Expression {
        Expression::Equality(Equality::Comparison(Comparison::Term(Term::Factor(
            Factor::Unary(Unary::Primary(Primary::GroupedExpression(Box::new(expr)))),
        ))))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct StringLiteral(String);

impl AsRef<str> for StringLiteral {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for StringLiteral {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct NumLiteral(f64);

impl From<f64> for NumLiteral {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl AsRef<f64> for NumLiteral {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Eq for NumLiteral {}
