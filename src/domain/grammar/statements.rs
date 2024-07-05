use super::{Block, Expression, StringLiteral};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Declaration {
    Declaration(VarDeclaration),
    FuncDeclaration(FunDeclaration),
    Statement(Statement),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum VarDeclaration {
    Declare(StringLiteral),
    DeclareAndAssign(StringLiteral, Expression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct FunDeclaration {
    pub(crate) name: String,
    pub(crate) arguments: Vec<String>,
    pub(crate) body: Block,
}

impl FunDeclaration {
    ///
    /// Mostly for testing purposes
    ///
    pub(crate) fn empty_named(name: String) -> Self {
        Self {
            name,
            arguments: Default::default(),
            body: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Statement {
    Expression(Expression),
    FinalExpression(Expression),
    Print(Expression),
    Assignment(StringLiteral, Expression),
}
