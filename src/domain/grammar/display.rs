use std::fmt::Display;

use super::{
    Comparison, Equality, Expression, Factor, LogicAnd, LogicOr, NumLiteral, Primary,
    StringLiteral, Term, Unary,
};

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::LogicOr(e) => write!(f, "{e}"),
        }
    }
}

impl Display for LogicOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicOr::LogicAnd(l) => write!(f, "{l}"),
            LogicOr::Or { left, right } => write!(f, "(OR {left} {right})"),
        }
    }
}

impl Display for LogicAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicAnd::Equality(e) => write!(f, "{e}"),
            LogicAnd::And { left, right } => write!(f, "(AND {left} {right})"),
        }
    }
}

impl Display for Equality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equality::Comparison(c) => write!(f, "{c}"),
            Equality::EqualityCheck { left, right } => write!(f, "(== {left} {right})"),
            Equality::InequalityCheck { left, right } => write!(f, "(!= {left} {right})"),
        }
    }
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comparison::Term(t) => write!(f, "{t}"),
            Comparison::Greater { left, right } => write!(f, "(> {left} {right})"),
            Comparison::GreaterEqual { left, right } => write!(f, "(>= {left} {right})"),
            Comparison::Less { left, right } => write!(f, "(< {left} {right})"),
            Comparison::LessEqual { left, right } => write!(f, "(<= {left} {right})"),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Factor(factor) => write!(f, "{factor}"),
            Term::Addition { left, right } => write!(f, "(+ {left} {right})"),
            Term::Subtraction { left, right } => write!(f, "(- {left} {right})"),
        }
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Factor::Unary(u) => write!(f, "{u}"),
            Factor::Multiplication { left, right } => write!(f, "(* {left} {right})"),
            Factor::Division { left, right } => write!(f, "(/ {left} {right})"),
        }
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unary::Primary(p) => write!(f, "{p}"),
            Unary::LogicalNegation(u) => write!(f, "(! {u})"),
            Unary::ArithmNegation(u) => write!(f, "(- {u})"),
        }
    }
}

impl Display for Primary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primary::Number(n) => write!(f, "{n}"),
            Primary::String(s) => write!(f, "'{s}'"),
            Primary::Identifier(i) => write!(f, "{i}"),
            Primary::True(_) => write!(f, "true"),
            Primary::False(_) => write!(f, "false"),
            Primary::Nil(_) => write!(f, "nil"),
            Primary::GroupedExpression(e) => write!(f, "(group {e})"),
        }
    }
}

impl Display for NumLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{n}", n = self.as_ref())
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{s}", s = self.as_ref())
    }
}

#[cfg(test)]
mod test {
    use crate::domain::{
        grammar::{Expression, Factor, NumLiteral, Primary, Unary},
        location::Location,
    };

    #[test]
    fn expression_displayed_correctly() {
        let expected = "(* (- 123) (group 45.67))".to_owned();
        let number: Expression =
            Primary::Number(NumLiteral::new(45.67, Location::default())).into();
        let grouped_expr = Primary::GroupedExpression(Box::new(number)); // ( 45.67 )

        let negated_number = Unary::ArithmNegation(Box::new(Unary::Primary(Primary::Number(
            NumLiteral::new(123.0, Location::default()),
        )))); // - 123s

        let overall: Expression = Factor::Multiplication {
            left: Box::new(Factor::Unary(negated_number)),
            right: Unary::Primary(grouped_expr),
        }
        .into();

        let actual = format!("{overall}");
        assert_eq!(expected, actual);
    }
}
