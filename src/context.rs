use std::collections::HashMap;

pub struct Context {
    url_params: HashMap<String, String>
}

impl Context {
    pub fn param(&self, name: &str) -> Option<&String> {
        self.url_params.get(name)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            url_params: HashMap::new(),
        }
    }
}