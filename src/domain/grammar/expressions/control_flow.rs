use crate::domain::grammar::{Block, Declaration};

use super::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct IfThen {
    pub(crate) condition: Box<Expression>,
    pub(crate) then: Box<Declaration>,
}

impl IfThen {
    pub(crate) fn new(condition: Expression, then: Declaration) -> Self {
        Self {
            condition: Box::new(condition),
            then: Box::new(then),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct While {
    pub(crate) condition: Box<Expression>,
    pub(crate) block: Box<Declaration>,
}

impl While {
    pub(crate) fn new(condition: Expression, block: Declaration) -> Self {
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
    pub(crate) for_block: Box<Declaration>,
}

impl DesugeredFor {
    pub(crate) fn new(for_block: Declaration) -> Self {
        Self {
            for_block: Box::new(for_block),
        }
    }
}
