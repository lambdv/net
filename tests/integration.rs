use std::collections::HashMap;
use web::models::http::{HTTPRequest, HTTPResponse, HTTPMethod};
use web::router::Router;

#[test]
fn test_parse_url() {
    let (path, query) = web::router::parse_url("/users?name=bob&age=10");
    assert_eq!(path, "/users");
    assert_eq!(query.get("name"), Some(&"bob".to_string()));
    assert_eq!(query.get("age"), Some(&"10".to_string()));

    let (path2, query2) = web::router::parse_url("/test");
    assert_eq!(path2, "/test");
    assert!(query2.is_empty());
}

#[test]
fn test_match_route() {
    let params = web::router::match_route("/posts/{id}", "/posts/1");
    assert_eq!(params.unwrap().get("id"), Some(&"1".to_string()));

    let params2 = web::router::match_route("/posts/{id}", "/posts/1/extra");
    assert!(params2.is_none());

    let params3 = web::router::match_route("/users", "/users");
    assert!(params3.unwrap().is_empty());
}

#[test]
fn test_router_with_params() {
    let mut router = Router::new();

    router.bind((HTTPMethod::GET, "/posts/{id}".to_string()), |req, res, pattern| {
        let path_params = req.path_params(pattern).unwrap();
        res.body = Some(format!("Post {}", path_params["id"]));
    });

    let req = HTTPRequest {
        method: HTTPMethod::GET,
        url: "/posts/42".to_string(),
        version: web::models::http::HTTPVersion::HTTP1_1,
        headers: HashMap::new(),
        body: None,
    };

    let mut res = HTTPResponse::default();

    router.handle(HTTPMethod::GET, &req, &mut res).unwrap();
    assert_eq!(res.body, Some("Post 42".to_string()));
}

#[test]
fn test_http_request_parsing() {
    let request_str = "GET /posts/123?name=test HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let req = HTTPRequest::new(request_str.to_string());
    assert_eq!(req.method, HTTPMethod::GET);
    assert_eq!(req.url, "/posts/123?name=test");
}