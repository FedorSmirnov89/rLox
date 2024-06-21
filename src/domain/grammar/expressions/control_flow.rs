use crate::domain::grammar::Block;

use super::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct IfThen {
    pub(crate) condition: Box<Expression>,
    pub(crate) then: Box<Block>,
}

impl IfThen {
    pub(crate) fn new(condition: Expression, then: Block) -> Self {
        Self {
            condition: Box::new(condition),
            then: Box::new(then),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct IfThenElse {
    pub(crate) if_then: IfThen,
    pub(crate) else_block: Box<Block>,
}

impl IfThenElse {
    pub(crate) fn new(if_then: IfThen, else_block: Block) -> Self {
        Self {
            if_then,
            else_block: Box::new(else_block),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct While {
    pub(crate) condition: Box<Expression>,
    pub(crate) block: Box<Block>,
}

impl While {
    pub(crate) fn new(condition: Expression, block: Block) -> Self {
        Self {
            condition: Box::new(condition),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct DesugeredFor {
    pub(crate) for_block: Box<Block>,
}

impl DesugeredFor {
    pub(crate) fn new(for_block: Block) -> Self {
        Self {
            for_block: Box::new(for_block),
        }
    }
}
