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
    IfThen(IfThen),
    IfThenElse(IfThenElse),
    While(While),
    For(DesugeredFor),
}

#[derive(Debug)]
pub(crate) struct Block(Vec<Declaration>);

impl Block {
    pub(crate) fn into_inner(self) -> Vec<Declaration> {
        self.0
    }
}

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

#[derive(Debug)]
pub(crate) struct IfThen {
    pub(crate) condition: Expression,
    pub(crate) then: Box<Declaration>,
}

impl IfThen {
    pub(crate) fn new(condition: Expression, then: Declaration) -> Self {
        Self {
            condition,
            then: Box::new(then),
        }
    }
}

#[derive(Debug)]
pub(crate) struct IfThenElse {
    pub(crate) if_then: IfThen,
    pub(crate) else_block: Box<Declaration>,
}

impl IfThenElse {
    pub(crate) fn new(if_then: IfThen, else_block: Declaration) -> Self {
        Self {
            if_then,
            else_block: Box::new(else_block),
        }
    }
}

#[derive(Debug)]
pub(crate) struct While {
    pub(crate) condition: Expression,
    pub(crate) block: Box<Declaration>,
}

impl While {
    pub(crate) fn new(condition: Expression, block: Declaration) -> Self {
        Self {
            condition,
            block: Box::new(block),
        }
    }
}

#[derive(Debug)]
pub(crate) struct For {
    pub(crate) init: Block,
    pub(crate) condition: Expression,
    pub(crate) update: Block,
    pub(crate) block: Block,
}

impl For {
    pub(crate) fn new(init: Block, condition: Expression, update: Block, block: Block) -> Self {
        Self {
            init,
            condition,
            update,
            block,
        }
    }
}

#[derive(Debug)]
pub(crate) struct DesugeredFor {
    pub(crate) for_block: Box<Declaration>,
}

impl DesugeredFor {
    pub(crate) fn new(for_block: Declaration) -> Self {
        Self {
            for_block: Box::new(for_block),
        }
    }
}
