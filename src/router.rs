pub type HTTPRoute = (crate::models::http::HTTPMethod, String);
pub type HTTPHandler = fn(crate::models::http::HTTPRequest, &mut crate::models::http::HTTPResponse);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Router {
    routes: std::collections::HashMap<HTTPRoute, HTTPHandler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: std::collections::HashMap::new(),
        }
    }

    pub fn bind(mut self, route: HTTPRoute, handler: HTTPHandler) -> Self {
        self.routes.insert(route, handler);
        self
    }
    pub fn handle(&self, route: HTTPRoute) -> Result<(), std::string::String> {
        Ok(())
    }
}

//route param = /posts/id/20
// route query = /posts?=id=10&user="howard"

// pub fn parse_route_schema(s: String) -> String {

// }
