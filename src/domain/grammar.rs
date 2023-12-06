use std::fmt::Display;

struct StringLiteral(String);

impl AsRef<str> for StringLiteral {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{s}", s = self.as_ref())
    }
}

struct NumLiteral(f64);

impl AsRef<f64> for NumLiteral {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Display for NumLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{n}", n = self.as_ref())
    }
}

enum Literal {
    Number(NumLiteral),
    String(StringLiteral),
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{n}"),
            Literal::String(s) => write!(f, "{s}"),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

enum Operator {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Division,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::EqualEqual => "==",
            Operator::BangEqual => "!=",
            Operator::Less => "<",
            Operator::LessEqual => "<=",
            Operator::Greater => ">",
            Operator::GreaterEqual => ">=",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Division => "/",
        };
        write!(f, "{s}")
    }
}

enum Expression {
    Literal(Literal),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Unary(u) => write!(f, "{u}"),
            Expression::Binary(b) => write!(f, "{b}"),
            Expression::Grouping(g) => write!(f, "{g}"),
        }
    }
}

enum Unary {
    ArthNegation(Expression),
    LogicNegation(Expression),
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (s, e) = match self {
            Unary::ArthNegation(e) => ("-", e),
            Unary::LogicNegation(e) => ("!", e),
        };
        write!(f, "({s} {e})")
    }
}

struct Grouping(Expression);

impl AsRef<Expression> for Grouping {
    fn as_ref(&self) -> &Expression {
        &self.0
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {e})", e = self.as_ref())
    }
}

struct Binary {
    left: Expression,
    operator: Operator,
    right: Expression,
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({operator} {left} {right})",
            operator = self.operator,
            left = self.left,
            right = self.right
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Binary, Expression, Grouping, Literal, NumLiteral, Operator, Unary};

    #[test]
    fn expression_displayed_correctly() {
        let expected = "(* (- 123) (group 45.67))".to_owned();
        let unary =
            Unary::ArthNegation(Expression::Literal(Literal::Number(NumLiteral(123 as f64))));
        let left = Expression::Unary(Box::new(unary));
        let grouping = Grouping(Expression::Literal(Literal::Number(NumLiteral(45.67))));
        let right = Expression::Grouping(Box::new(grouping));
        let binary = Binary {
            operator: Operator::Star,
            left,
            right,
        };
        let expr = Expression::Binary(Box::new(binary));
        let actual = format!("{expr}");
        assert_eq!(expected, actual);
    }
}
