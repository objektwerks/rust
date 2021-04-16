#![deny(warnings)]

use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, Rust!")))
}

#[tokio::main]
/** To test server: curl http://localhost:7979/ */
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn( |_conn| {
        async { Ok::<_, Infallible>( service_fn(hello) ) }
    });

    let addr = ([127, 0, 0, 1], 7979).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}