use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, fn() -> String>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, handler: fn() -> String) {
        self.routes.insert(path.to_string(), handler);
    }

    pub fn handle_request(&self, path: &str) -> Option<String> {
        match self.routes.get(path) {
            Some(handler) => Some(handler()),
            None => None,
        }
    }
}