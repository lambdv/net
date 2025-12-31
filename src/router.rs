/// Match path against pattern, extract params. Pattern like "/posts/{id}"
pub fn parse_url(url: &str) -> (String, std::collections::HashMap<String, String>) {
    let mut query_params = std::collections::HashMap::new();
    if let Some((path, query)) = url.split_once('?') {
        for pair in query.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                query_params.insert(key.to_string(), value.to_string());
            }
        }
        (path.to_string(), query_params)
    } else {
        (url.to_string(), query_params)
    }
}

pub fn match_route(pattern: &str, path: &str) -> Option<std::collections::HashMap<String, String>> {
    let pattern_parts: Vec<&str> = pattern.trim_matches('/').split('/').collect();
    let path_parts: Vec<&str> = path.trim_matches('/').split('/').collect();

    if pattern_parts.len() != path_parts.len() {
        return None;
    }

    let mut params = std::collections::HashMap::new();

    for (pat, p) in pattern_parts.iter().zip(path_parts) {
        if pat.starts_with('{') && pat.ends_with('}') {
            let param_name = &pat[1..pat.len() - 1];
            params.insert(param_name.to_string(), p.to_string());
        } else if *pat != p {
            return None;
        }
    }

    Some(params)
}

pub type HTTPRoute = (crate::models::http::HTTPMethod, String);
pub type HTTPHandler = Box<dyn Fn(&crate::models::http::HTTPRequest, &mut crate::models::http::HTTPResponse, &str) + Send + Sync>;

pub struct Router {
    routes: std::collections::HashMap<HTTPRoute, HTTPHandler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: std::collections::HashMap::new(),
        }
    }

    pub fn bind<F>(&mut self, route: HTTPRoute, handler: F)
    where
        F: Fn(&crate::models::http::HTTPRequest, &mut crate::models::http::HTTPResponse, &str) + 'static + Send + Sync,
    {
        self.routes.insert(route, Box::new(handler));
    }
    pub fn handle(&self, method: crate::models::http::HTTPMethod, request: &crate::models::http::HTTPRequest, response: &mut crate::models::http::HTTPResponse) -> Result<(), std::string::String> {
        for ((route_method, route_path), handler) in &self.routes {
            if *route_method == method {
                if request.path_params(route_path).is_some() {
                    handler(request, response, route_path);
                    return Ok(());
                }
            }
        }
        Err("Route not found".to_string())
    }
}
