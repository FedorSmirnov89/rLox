use super::{Expression, StringLiteral};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Declaration {
    Declaration(VarDeclaration),
    Statement(Statement),
    Block(Block),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum VarDeclaration {
    Declare(StringLiteral),
    DeclareAndAssign(StringLiteral, Expression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Statement {
    Expression(Expression),
    Print(Expression),
    Assignment(StringLiteral, Expression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Block {
    pub(crate) statements: Vec<Declaration>,
    pub(crate) final_expression: Option<Expression>,
}

impl Into<Block> for Vec<Declaration> {
    fn into(self) -> Block {
        Block {
            statements: self,
            final_expression: None,
        }
    }
}
