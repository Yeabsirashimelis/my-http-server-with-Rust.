use crate::http::{request::Request, response::Response};
use crate::HttpCode;
use std::collections::HashMap;

/// Type alias for route handler functions
type Handler = Box<dyn Fn(&Request, &HashMap<String, String>) -> Response + Send + Sync>;

//ONLY SUPPORT GET FOR NOW
pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    /// Create a new empty router
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Register a GET route
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request, &HashMap<String, String>) -> Response + Send + Sync + 'static,
    {
        self.routes.insert(path.into(), Box::new(handler));
    }

    /// Handle an incoming request
    pub fn handle(&self, req: &Request) -> Response {
        // First, try exact match
        if let Some(handler) = self.routes.get(&req.path) {
            return handler(req, &HashMap::new());
        }

        // Then try dynamic routes
        for (pattern, handler) in &self.routes {
            if pattern.contains(":") {
                if let Some(params) = Self::match_dynamic(pattern, &req.path) {
                    return handler(req, &params);
                }
            }
        }

        // Not found
        Response::new(HttpCode::NotFound, "404 - Not Found")
    }

    /// Match a dynamic route like `/user/:id` with `/user/42`
    /// Returns a map of param names to values if matched
    fn match_dynamic(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (p, v) in pattern_parts.iter().zip(path_parts.iter()) {
            if p.starts_with(':') {
                // dynamic parameter
                let key = p.trim_start_matches(':').to_string();
                params.insert(key, (*v).to_string());
            } else if p != v {
                // static segment does not match
                return None;
            }
        }

        Some(params)
    }
}
