use crate::http::{method::Method, request::Request, response::Response};
use crate::HttpCode;
use std::collections::HashMap;

/// Type alias for route handler functions
type Handler = Box<dyn Fn(&Request, &HashMap<String, String>) -> Response + Send + Sync>;

/// Router that supports multiple HTTP methods
pub struct Router {
    routes: HashMap<Method, HashMap<String, Handler>>,
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
        self.insert(Method::GET, path, handler);
    }

    /// Register a POST route
    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request, &HashMap<String, String>) -> Response + Send + Sync + 'static,
    {
        self.insert(Method::POST, path, handler);
    }

    /// Register a PUT route
    pub fn put<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request, &HashMap<String, String>) -> Response + Send + Sync + 'static,
    {
        self.insert(Method::PUT, path, handler);
    }

    /// Register a DELETE route
    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request, &HashMap<String, String>) -> Response + Send + Sync + 'static,
    {
        self.insert(Method::DELETE, path, handler);
    }

    /// Internal helper to insert a handler
    fn insert<F>(&mut self, method: Method, path: &str, handler: F)
    where
        F: Fn(&Request, &HashMap<String, String>) -> Response + Send + Sync + 'static,
    {
        let method_routes = self.routes.entry(method).or_insert_with(HashMap::new);
        method_routes.insert(path.into(), Box::new(handler));
    }

    /// Handle an incoming request
    pub fn handle(&self, req: &Request) -> Response {
        if let Some(method_routes) = self.routes.get(&req.method) {
            // Try exact match
            if let Some(handler) = method_routes.get(&req.path) {
                return handler(req, &HashMap::new());
            }

            // Try dynamic routes
            for (pattern, handler) in method_routes {
                if pattern.contains(":") {
                    if let Some(params) = Self::match_dynamic(pattern, &req.path) {
                        return handler(req, &params);
                    }
                }
            }
        }

        // Not found
        Response::new(HttpCode::NotFound, "404 - Not Found")
    }

    /// Match a dynamic route like `/user/:id` with `/user/42`
    fn match_dynamic(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (p, v) in pattern_parts.iter().zip(path_parts.iter()) {
            if p.starts_with(':') {
                let key = p.trim_start_matches(':').to_string();
                params.insert(key, (*v).to_string());
            } else if p != v {
                return None;
            }
        }

        Some(params)
    }
}
