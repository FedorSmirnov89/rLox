use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::{domain::grammar::FunDeclaration, Value};

#[derive(Debug)]
pub(super) struct Scope {
    outer: Option<Box<Self>>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, FunDeclaration>,
}

impl Scope {
    pub(super) fn new_inner(outer: Self) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            variables: HashMap::default(),
            functions: HashMap::default(),
        }
    }

    pub(super) fn collapse(self) -> Self {
        *self.outer.expect("cannot collapse global scope")
    }

    pub(super) fn declare_var(&mut self, iden: impl Into<String>) {
        let key = iden.into();
        self.variables.insert(key, Value::nil());
    }

    pub(super) fn set_var_value(&mut self, iden: impl Into<String>, val: Value) -> Result<()> {
        let key = iden.into();
        let Some(var_scope) = self.variable_scope(&key) else {
            bail!("variable not declared")
        };
        var_scope.variables.insert(key, val);
        Ok(())
    }

    pub(super) fn get_var_value(&self, iden: &str) -> Option<&Value> {
        match self.variables.get(iden) {
            Some(v) => Some(v),
            None => match &self.outer {
                Some(outer) => outer.get_var_value(iden),
                None => None,
            },
        }
    }

    pub(super) fn declare_fun(
        &mut self,
        iden: impl Into<String>,
        fun_declaration: FunDeclaration,
    ) -> Result<()> {
        let key = iden.into();
        if self.functions.contains_key(&key) {
            bail!("function already declared")
        } else {
            self.functions.insert(key, fun_declaration);
            Ok(())
        }
    }

    pub(super) fn get_fun(&self, iden: &str) -> Option<&FunDeclaration> {
        match self.functions.get(iden) {
            Some(v) => Some(v),
            None => match &self.outer {
                Some(outer) => outer.get_fun(iden),
                None => None,
            },
        }
    }

    fn new_global() -> Self {
        Self {
            outer: None,
            variables: HashMap::default(),
            functions: HashMap::default(),
        }
    }

    fn variable_scope(&mut self, iden: &str) -> Option<&mut Self> {
        match self.variables.get_mut(iden) {
            Some(_) => Some(self),
            None => match &mut self.outer {
                Some(outer) => outer.variable_scope(iden),
                None => None,
            },
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new_global()
    }
}

#[cfg(test)]
mod test {

    use crate::{domain::location::CodeSpan, Environment, ValueType};

    use super::*;

    #[test]
    fn scoped_vars_shadow_globals() {
        let mut env = Environment::default();
        env.declare_var("a");
        env.set_var_value("a", Value::new(ValueType::Number(1.0), CodeSpan::default()))
            .unwrap();
        env.new_inner_scope();
        env.declare_var("a");
        env.set_var_value("a", Value::new(ValueType::Number(2.0), CodeSpan::default()))
            .unwrap();
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Number(2.0)
        );
        env.teardown_inner_scope();
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Number(1.0)
        );
    }

    #[test]
    fn test_mutate_outer_scope() {
        // Arrange - global var a; value is 1
        let mut env = Environment::default();
        env.declare_var("a");
        env.set_var_value("a", Value::new(ValueType::Number(1.0), CodeSpan::default()))
            .unwrap();

        // Act I - setup inner scope; define b = 2
        env.new_inner_scope();
        env.declare_var("b");
        env.set_var_value("b", Value::new(ValueType::Number(2.0), CodeSpan::default()))
            .unwrap();

        // Assert I - check b is 2 and a is 1
        assert_eq!(
            env.get_var_value("b").unwrap().v_type,
            ValueType::Number(2.0)
        );
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Number(1.0)
        );

        // Act II - mutate a to true
        env.set_var_value(
            "a",
            Value::new(ValueType::Boolean(true), CodeSpan::default()),
        )
        .unwrap();

        // Assert II - check a is true
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Boolean(true)
        );

        // Act III - teardown inner scope
        env.teardown_inner_scope();

        // Assert III - check a is still true and b is not declared
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Boolean(true)
        );
        assert!(env.get_var_value("b").is_none());
    }

    #[test]
    fn test_variable_shadowing() {
        // Arrange - global var a; value is 1
        let mut env = Environment::default();

        // Act I - define a = 1 (global)
        env.declare_var("a");
        env.set_var_value("a", Value::new(ValueType::Number(1.0), CodeSpan::default()))
            .unwrap();

        // Assert I - check a is 1
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Number(1.0)
        );

        // Act II - setup inner scope; define a = true
        env.new_inner_scope();
        env.declare_var("a");
        env.set_var_value(
            "a",
            Value::new(ValueType::Boolean(true), CodeSpan::default()),
        )
        .unwrap();

        // Assert II - check a is true
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Boolean(true)
        );

        // Act III - teardown inner scope
        env.teardown_inner_scope();

        // Assert III - check that a is still 1
        assert_eq!(
            env.get_var_value("a").unwrap().v_type,
            ValueType::Number(1.0)
        );
    }

    #[test]
    fn declared_function_is_accessible() {
        // Arrange
        let mut env = Environment::default();
        let fun_decl = FunDeclaration::empty_named("my_fun".to_owned());

        // Act - add function
        env.declare_fun("my_fun", fun_decl.clone())
            .expect("failed to declare function");

        // Assert check that environment has the function
        assert_eq!(env.get_fun_block("my_fun").unwrap(), &fun_decl);
    }

    #[test]
    fn functions_respect_scope() {
        // Arrange - env with an inner scope and a function block
        let mut env = Environment::default();
        let fun = FunDeclaration::empty_named("my_fun".to_owned());
        env.new_inner_scope();

        // Act - add function in inner scope
        env.declare_fun("my_fun", fun.clone())
            .expect("failed to declare function");

        // Assert I - check that function is accessible
        assert_eq!(env.get_fun_block("my_fun").unwrap(), &fun);

        // Act II - teardown inner scope
        env.teardown_inner_scope();

        // Assert II - check that function is not accessible
        assert!(env.get_fun_block("my_fun").is_none());
    }

    #[test]
    fn cannot_declare_same_fun_twice() {
        // Arrange - env with a function block
        let mut env = Environment::default();
        let fun = FunDeclaration::empty_named("my_fun".to_owned());

        // Act - add function twice
        env.declare_fun("my_fun", fun.clone())
            .expect("failed to declare function");
        let result = env.declare_fun("my_fun", fun.clone());

        // Assert - check that second declaration fails
        assert!(result.is_err());
    }
}
