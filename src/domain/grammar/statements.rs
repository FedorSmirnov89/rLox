use super::{Expression, StringLiteral};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Declaration {
    Declaration(VarDeclaration),
    Statement(Statement),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum VarDeclaration {
    Declare(StringLiteral),
    DeclareAndAssign(StringLiteral, Expression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Statement {
    Expression(Expression),
    FinalExpression(Expression),
    Print(Expression),
    Assignment(StringLiteral, Expression),
}
