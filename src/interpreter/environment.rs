use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::Value;

///
/// The state of the interpreter:
///
/// - The current values of the global variables
///
#[derive(Debug)]
pub struct Environment {
    scope: Option<Scope>,
    tmp_value: Option<Value>,
}

impl Environment {
    pub fn set_tmp_value(&mut self, val: Value) {
        self.tmp_value = Some(val)
    }

    pub fn get_tmp_value(&self) -> Option<&Value> {
        self.tmp_value.as_ref()
    }

    pub fn declare_var(&mut self, iden: impl Into<String>) {
        self.scope_mut().declare_var(iden);
    }

    pub fn set_var_value(&mut self, iden: impl Into<String>, val: Value) -> Result<()> {
        self.scope_mut().set_var_value(iden, val)
    }

    pub fn get_var_value(&self, iden: &str) -> Option<&Value> {
        self.scope().get_var_value(iden)
    }

    ///
    /// Creates a new inner scope in the current scope
    ///
    pub fn new_inner_scope(&mut self) {
        let outer = self.scope.take().expect("scope must be set");
        self.scope = Some(Scope::new_inner(outer));
    }

    ///
    /// Tears down the current innermost scope
    ///
    pub fn teardown_inner_scope(&mut self) {
        let inner = self.scope.take().expect("scope must be set");
        self.scope = Some(inner.collapse());
    }

    fn scope(&self) -> &Scope {
        self.scope.as_ref().expect("scope should always be set")
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.scope.as_mut().expect("scope should always be set")
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            scope: Some(Scope::default()),
            tmp_value: None,
        }
    }
}

#[derive(Debug)]
struct Scope {
    outer: Option<Box<Self>>,
    variables: HashMap<String, Value>,
}

impl Scope {
    fn new_global() -> Self {
        Self {
            outer: None,
            variables: HashMap::default(),
        }
    }

    fn new_inner(outer: Self) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            variables: HashMap::default(),
        }
    }

    fn collapse(self) -> Self {
        *self.outer.expect("cannot collapse global scope")
    }

    fn declare_var(&mut self, iden: impl Into<String>) {
        let key = iden.into();
        self.variables.insert(key, Value::nil());
    }

    fn set_var_value(&mut self, iden: impl Into<String>, val: Value) -> Result<()> {
        let key = iden.into();
        let Some(var_scope) = self.variable_scope(&key) else {
            bail!("variable not declared")
        };
        var_scope.variables.insert(key, val);
        Ok(())
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

    fn get_var_value(&self, iden: &str) -> Option<&Value> {
        match self.variables.get(iden) {
            Some(v) => Some(v),
            None => match &self.outer {
                Some(outer) => outer.get_var_value(iden),
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

    use crate::{domain::location::CodeSpan, ValueType};

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
}
