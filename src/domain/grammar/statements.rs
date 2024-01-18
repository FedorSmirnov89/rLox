use super::{Expression, StringLiteral};

#[derive(Debug)]
pub(crate) enum Declaration {
    Declaration(VarDeclaration),
    Statement(Statement),
}

#[derive(Debug)]
pub(crate) enum VarDeclaration {
    Declare(StringLiteral),
    DeclareAndAssign(StringLiteral, Expression),
}

#[derive(Debug)]
pub(crate) enum Statement {
    Expression(Expression),
    Print(Expression),
    Assignment(StringLiteral, Expression),
}
