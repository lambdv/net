use web::models::http::{HTTPMethod, HTTPResponse};
///example
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut router = web::router::Router::new();

    router.bind(
        (HTTPMethod::GET, String::from("/test")),
        |req, res, pattern| {
            println!("{}", req);
            let path_params = req.path_params(pattern);
            let query_params = req.query_params();
            println!("Path params: {:?}", path_params);
            println!("Query params: {:?}", query_params);
            *res = HTTPResponse::default();
        },
    );

    router.bind(
        (HTTPMethod::GET, String::from("/posts/{id}")),
        |req, res, pattern| {
            let path_params = req.path_params(pattern).unwrap();
            println!("Post ID: {:?}", path_params.get("id"));
            *res = HTTPResponse::default();
            res.body = Some(format!("Post {}", path_params["id"]));
        },
    );

    router.bind(
        (HTTPMethod::GET, String::from("/users")),
        |req, res, _pattern| {
            let query_params = req.query_params();
            println!("Query params: {:?}", query_params);
            *res = HTTPResponse::default();
            res.body = Some(format!(
                "Users with name: {:?}, age: {:?}",
                query_params.get("name"),
                query_params.get("age")
            ));
        },
    );

    web::httpserver::HTTPServer::new(3000, router, std::collections::HashMap::new())
        .start()
        .await?;
    Ok(())
}
