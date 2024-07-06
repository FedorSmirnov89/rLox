use anyhow::Result;

use crate::{domain::grammar::FunDeclaration, Value};

use self::scope::Scope;

mod callables;
mod scope;

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

    pub(crate) fn declare_fun(
        &mut self,
        iden: impl Into<String>,
        fun_declaration: FunDeclaration,
    ) -> Result<()> {
        self.scope_mut().declare_fun(iden, fun_declaration)
    }

    #[cfg(test)]
    pub(crate) fn get_fun_block(&mut self, iden: &str) -> Option<&FunDeclaration> {
        self.scope().get_fun(iden)
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
