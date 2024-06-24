use anyhow::Result;

use crate::{
    domain::{
        grammar::{
            control_flow::{DesugeredFor, For, IfThen, IfThenElse, While},
            Declaration, Expression, Statement,
        },
        scanning::TokenType,
    },
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    pub(crate) fn expression(&mut self) -> Result<Expression> {
        match self.expression_type()? {
            ExpressionType::If => self.if_expression(),
            ExpressionType::While => self.while_expression(),
            ExpressionType::For => self.for_expression(),
            ExpressionType::Block => self.block_expression(),
            ExpressionType::OtherExpression => self.other_expression(),
        }
    }

    fn block_expression(&mut self) -> Result<Expression> {
        let block = self.block()?;
        Ok(Expression::Block(Box::new(block)))
    }

    fn if_expression(&mut self) -> Result<Expression> {
        self.advance(); // consume the if
        let condition = self.expression()?;
        let then_block = self.block()?;
        let if_then = IfThen::new(condition, then_block);
        if self.on_else_branch()? {
            self.if_then_else_expression(if_then)
        } else {
            Ok(Expression::IfThen(if_then))
        }
    }

    fn if_then_else_expression(&mut self, if_then: IfThen) -> Result<Expression> {
        self.advance(); // consume the else
        let else_block = self.block()?;
        let if_then_else = IfThenElse::new(if_then, else_block);
        Ok(Expression::IfThenElse(if_then_else))
    }

    fn while_expression(&mut self) -> Result<Expression> {
        self.advance(); // consume the while
        let condition = self.expression()?;
        let block = self.block()?;
        let while_statement = While::new(condition, block);
        Ok(Expression::While(while_statement))
    }

    fn for_expression(&mut self) -> Result<Expression> {
        let for_statement = self.raw_for_expression()?;
        let desugered_for = desugered_for(for_statement);
        Ok(Expression::For(desugered_for))
    }

    fn raw_for_expression(&mut self) -> Result<For> {
        self.advance(); // consume the for
        let init = self.read_block_content()?;
        self.expect(&TokenType::BraceLeft, "opening bracket for condition")?;
        self.advance(); // consume the opening bracket of condition
        let condition = self.expression()?;
        self.expect(&TokenType::BraceRight, "closing bracket for condition")?;
        self.advance(); // consume the closing bracket of condition
        let update = self.read_block_content()?;
        let block = self.read_block_content()?;
        let for_statement = For::new(init, condition, update, block);
        Ok(for_statement)
    }

    fn other_expression(&mut self) -> Result<Expression> {
        let l_or = self.logic_or()?;
        Ok(Expression::LogicOr(l_or))
    }

    fn expression_type(&self) -> Result<ExpressionType> {
        if self.on_if_expression()? {
            Ok(ExpressionType::If)
        } else if self.on_while_expression()? {
            Ok(ExpressionType::While)
        } else if self.on_for_expression()? {
            Ok(ExpressionType::For)
        } else if self.at_start_of_block()? {
            Ok(ExpressionType::Block)
        } else {
            Ok(ExpressionType::OtherExpression)
        }
    }

    fn on_if_expression(&self) -> Result<bool> {
        let current_t = self.current()?.t_type();
        if let TokenType::IF = current_t {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_else_branch(&self) -> Result<bool> {
        let current_t = self.current()?.t_type();
        if let TokenType::ELSE = current_t {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_while_expression(&self) -> Result<bool> {
        let current_t = self.current()?.t_type();
        if let TokenType::WHILE = current_t {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_for_expression(&self) -> Result<bool> {
        let current_t = self.current()?.t_type();
        if let TokenType::FOR = current_t {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

enum ExpressionType {
    OtherExpression,
    If,
    While,
    For,
    Block,
}

fn desugered_for(for_statement: For) -> DesugeredFor {
    let For {
        init,
        condition,
        update,
        block,
    } = for_statement;
    let mut while_declarations = block.statements;
    while_declarations.extend(update.statements);
    let while_block = while_declarations.into();
    let while_loop = While::new(condition, while_block);
    let mut init_declarations = init.statements;
    // append the while loop to the end of the init block
    init_declarations.push(Declaration::Statement(Statement::Expression(
        Expression::While(while_loop),
    )));
    let for_block = init_declarations.into();
    DesugeredFor::new(for_block)
}
