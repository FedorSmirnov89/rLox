use anyhow::Result;

use crate::{
    domain::{
        grammar::{
            Declaration, DesugeredFor, For, IfThen, IfThenElse, Statement, StringLiteral, While,
        },
        scanning::TokenType,
    },
    matches_t_type,
    parser::Parser,
};

impl<'tokens> Parser<'tokens> {
    ///
    /// Reads out an expression; Checks that it is followed by a semicolon. Also advances the current
    ///
    pub(crate) fn statement(&mut self) -> Result<Statement> {
        let statement = match self.current_statement()? {
            StatementType::Print => self.print_statement()?,
            StatementType::Expression => self.expression_statement()?,
            StatementType::Assignment => self.assignment_statement()?,
            StatementType::If => self.if_statement()?,
            StatementType::While => self.while_statement()?,
            StatementType::For => self.for_statement()?,
        };

        Ok(statement)
    }

    fn consume_semicolon(&mut self) -> Result<()> {
        self.expect(&TokenType::Semicolon, "semicolon after statement")?;
        self.advance();
        Ok(())
    }

    ///
    /// Used for both if and if-else statements
    ///
    fn if_statement(&mut self) -> Result<Statement> {
        self.advance(); // consume the if
        let condition = self.expression()?;
        let then_block = self.block()?;
        let if_then = IfThen::new(condition, then_block);
        if self.on_else_branch()? {
            self.if_then_else_statement(if_then)
        } else {
            Ok(Statement::IfThen(if_then))
        }
    }

    fn if_then_else_statement(&mut self, if_then: IfThen) -> Result<Statement> {
        self.advance(); // consume the else
        let else_block = self.block()?;
        let if_then_else = IfThenElse::new(if_then, else_block);
        Ok(Statement::IfThenElse(if_then_else))
    }

    fn for_statement(&mut self) -> Result<Statement> {
        let for_statement = self.raw_for_statement()?;
        let desugered_for = desugered_for(for_statement);
        Ok(Statement::For(desugered_for))
    }

    fn raw_for_statement(&mut self) -> Result<For> {
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

    fn while_statement(&mut self) -> Result<Statement> {
        self.advance(); // consume the while
        let condition = self.expression()?;
        let block = self.block()?;
        let while_statement = While::new(condition, block);
        Ok(Statement::While(while_statement))
    }

    fn print_statement(&mut self) -> Result<Statement> {
        self.advance();
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::Print(expr))
    }

    fn assignment_statement(&mut self) -> Result<Statement> {
        let literal = StringLiteral::identifier_from_token(self.current()?)?;
        self.advance();
        self.advance();
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::Assignment(literal, expr))
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Statement::Expression(expr))
    }

    fn current_statement(&self) -> Result<StatementType> {
        if self.on_print_statement()? {
            Ok(StatementType::Print)
        } else if self.on_if_statement()? {
            Ok(StatementType::If)
        } else if self.on_while_statement()? {
            Ok(StatementType::While)
        } else if self.on_assignment_statement()? {
            Ok(StatementType::Assignment)
        } else if self.on_for_statement()? {
            Ok(StatementType::For)
        } else {
            Ok(StatementType::Expression)
        }
    }

    fn on_for_statement(&self) -> Result<bool> {
        let current_t = self.current()?.t_type();
        if let TokenType::FOR = current_t {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_while_statement(&self) -> Result<bool> {
        let current_t = self.current()?.t_type();
        if let TokenType::WHILE = current_t {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_if_statement(&self) -> Result<bool> {
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

    fn on_assignment_statement(&self) -> Result<bool> {
        let current_t_type = self.current()?.t_type();
        let next_t_type = self.next()?.t_type();
        match (current_t_type, next_t_type) {
            (TokenType::Identifier(_), TokenType::Equal) => Ok(true),
            _ => Ok(false),
        }
    }

    fn on_print_statement(&self) -> Result<bool> {
        let current = self.current()?;
        Ok(matches_t_type!(current, &TokenType::PRINT))
    }
}

fn desugered_for(for_statement: For) -> DesugeredFor {
    let For {
        init,
        condition,
        update,
        block,
    } = for_statement;
    let mut while_declarations = block.into_inner();
    while_declarations.extend(update.into_inner());
    let while_block = Declaration::Block(while_declarations.into());
    let while_loop = While::new(condition, while_block);
    let mut init_declarations = init.into_inner();
    // append the while loop to the end of the init block
    init_declarations.push(Declaration::Statement(Statement::While(while_loop)));
    let for_block = Declaration::Block(init_declarations.into());
    DesugeredFor::new(for_block)
}

enum StatementType {
    Print,
    Assignment,
    Expression,
    // Used for both if and if-else statements
    If,
    While,
    For,
}
