use crate::val::Val;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    bindings: HashMap<String, Val>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.get_binding_value_without_error(name)
            .ok_or_else(|| format!("binding with name '{}' does not exist", name))
    }

    fn get_binding_value_without_error(&self, name: &str) -> Option<Val> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.get_binding_value_without_error(name))
        })
    }
}
