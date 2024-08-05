use crate::interpreter::RuntimeError;
use crate::parser::Object;
use crate::token::TokenType::VAR;
use std::collections::HashMap;

pub(crate) struct Environment {
    _map: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            _map: HashMap::new(),
            enclosing: None,
        }
    }
    pub fn get(&self, identifier: String) -> Result<&Object, RuntimeError> {
        self._map
            .get(&identifier)
            .or_else(|| {
                self.enclosing
                    .as_ref()
                    .and_then(|e| e.get(identifier.clone()).ok())
            })
            .ok_or_else(|| {
                RuntimeError::new(format!("Undefined variable {identifier}."), VAR)
            })
    }

    pub fn set(&mut self, identifier: String, object: Object) {
        self._map.insert(identifier.clone(), object.clone());

        if self.enclosing.is_some() {
            self.enclosing.as_mut().unwrap().set(identifier, object)
        }
    }
}
