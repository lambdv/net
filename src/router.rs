// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct RoutePattern {
//     segments: Vec<RouteSegment>,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// enum RouteSegment {
//     Static(String),
//     Param(String),
// }

// impl RoutePattern {
//     pub fn new(schema: &str) -> Self {
//         let segments = schema
//             .trim_matches('/')
//             .split('/')
//             .map(|s| {
//                 if s.starts_with('{') && s.ends_with('}') {
//                     RouteSegment::Param(s[1..s.len() - 1].to_string())
//                 } else {
//                     RouteSegment::Static(s.to_string())
//                 }
//             })
//             .collect();

//         Self { segments }
//     }
// }

// /// get params if the path follows the pattern, else returns none
// fn match_path(
//     pattern: &RoutePattern,
//     path: &str,
// ) -> Option<std::collections::HashMap<String, String>> {
//     let parts: Vec<&str> = path.trim_matches('/').split('/').collect();

//     if parts.len() != pattern.segments.len() {
//         return None;
//     }

//     let mut params = std::collections::HashMap::new();

//     for (seg, part) in pattern.segments.iter().zip(parts) {
//         match seg {
//             RouteSegment::Static(s) if s == part => {}
//             RouteSegment::Param(name) => {
//                 params.insert(name.clone(), part.to_string());
//             }
//             _ => return None,
//         }
//     }

//     Some(params)
// }

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

    pub fn bind(&mut self, route: HTTPRoute, handler: HTTPHandler) {
        self.routes.insert(route, handler);
    }
    pub fn handle(&self, route: HTTPRoute, request: crate::models::http::HTTPRequest, response: &mut crate::models::http::HTTPResponse) -> Result<(), std::string::String> {
        let handler = self.routes.get(&route).ok_or("Route not found")?;
        handler(request, response);
        Ok(())
    }
}
