use super::Expression;

#[derive(Debug)]
pub(crate) enum Declaration {
    Declaration(VarDeclaration),
    Statement(Statement),
}

#[derive(Debug)]
pub(crate) struct VarDeclaration {}

#[derive(Debug)]
pub(crate) enum Statement {
    Expression(Expression),
    Print(Expression),
}
