mod models;
mod router;
use models::http::{HTTPMethod, HTTPResponse};

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("server running on http://127.0.0.1:3000");

    let mut router = router::Router::new();

    router.bind((HTTPMethod::GET, String::from("/test")), |req, res| {
        println!("{}", req);
        *res = HTTPResponse::default();
    });

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream, &router)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, router: &router::Router) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    let size = stream.read(&mut buffer)?;
    let s = std::str::from_utf8(&buffer[..size]).unwrap();
    let data = models::http::HTTPRequest::new(s.to_string());
    let route = (data.method.clone(), data.url.clone());

    // println!("route: {:?}", route);

    // println!("data: {:?}", data);

    let mut res = HTTPResponse::default();
    let hmm = router.handle(route, data.clone(), &mut res);
    if let Err(e) = hmm {
        println!("Error: {}", e);
    }
    stream.write_all(res.to_string().as_bytes())?;
    stream.flush()?;
    Ok(())
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Dater {
//     x: i32,
//     y: i32,
// }
// fn handle_connection(mut stream: TcpStream) -> std::io::Result<Dater> {
//     let mut buffer = [0; 1024];
//     let size = stream.read(&mut buffer)?;
//     let s = std::str::from_utf8(&buffer[..size]).unwrap();
//     let data = serde_json::from_str::<Dater>(&s).unwrap();
//     println!("{:?}", data);

//     let res = Dater {
//         x: data.x + 1,
//         y: data.y + 2,
//     };
//     stream.write_all(serde_json::to_string(&res)?.as_bytes())?;
//     stream.flush()?;
//     Ok(data)
// }

    //just deserialize as a lossy json object
    //let body = serde_json::from_str::<serde_json::Value>(&data.body.unwrap()).unwrap();
    //println!("{}", body.to_string());