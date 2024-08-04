use crate::interpreter::RuntimeError;
use crate::parser::Object;
use crate::token::TokenType::VAR;
use std::collections::HashMap;

pub(crate) struct Environment {
    var_map: HashMap<String, Object>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            var_map: HashMap::new(),
        }
    }
    pub fn get_var(&self, identifier: String) -> Result<&Object, RuntimeError> {
        self.var_map.get(&identifier).ok_or_else(|| {
            RuntimeError::new(format!("Undefined variable {identifier}."), VAR)
        })
    }

    pub fn set_var(&mut self, identifier: String, object: Object) {
        self.var_map.insert(identifier, object);
    }
}
