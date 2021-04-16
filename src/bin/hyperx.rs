#![deny(warnings)]

use chrono::Utc;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

async fn now(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/now") => Ok( Response::new( Body::from( Utc::now().to_string() ) ) ),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
/** To test server: curl http://localhost:7878/now */
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 7878).into();
    let service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>( service_fn( now ) )
    });
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}