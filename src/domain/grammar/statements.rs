use super::{Expression, StringLiteral};

#[derive(Debug)]
pub(crate) enum Declaration {
    Declaration(VarDeclaration),
    Statement(Statement),
    Block(Block),
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

#[derive(Debug)]
pub(crate) struct Block(Vec<Declaration>);

impl AsRef<[Declaration]> for Block {
    fn as_ref(&self) -> &[Declaration] {
        &self.0
    }
}

impl Into<Block> for Vec<Declaration> {
    fn into(self) -> Block {
        Block(self)
    }
}
