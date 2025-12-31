use crate::models::http::HTTPResponse;
use crate::router;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct HTTPServer {
    port: i32,
    router: Arc<router::Router>,
    _context: std::collections::HashMap<String, String>,
}

impl HTTPServer {
    pub fn new(
        port: i32,
        router: router::Router,
        context: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            port,
            router: Arc::new(router),
            _context: context,
        }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        println!("Server running on http://127.0.0.1:{}", self.port);
        loop {
            let (socket, addr) = listener.accept().await?;

            let router = Arc::clone(&self.router);
            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket, router).await {
                    eprintln!("{}: {}", addr, e);
                }
            });
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    router: Arc<crate::router::Router>,
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    let size = stream.read(&mut buffer).await?;

    let s = std::str::from_utf8(&buffer[..size]).unwrap();
    let data = crate::models::http::HTTPRequest::new(s.to_string());

    let mut res = HTTPResponse::default();
    let hmm = router.handle(data.method.clone(), &data, &mut res);
    if let Err(e) = hmm {
        println!("Error: {}", e);
        res = HTTPResponse::error(crate::models::http::HTTPStatus::InternalServerError, &e);
        stream.write_all(res.to_string().as_bytes()).await?;
        return Ok(());
    }
    stream.write_all(res.to_string().as_bytes()).await?;
    Ok(())
}
