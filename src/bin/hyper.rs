#![deny(warnings)]

use chrono::Utc;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use std::convert::Infallible;

async fn now(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(format!("Now: {}", Utc::now().to_string()))))
}


/** To test server: curl http://localhost:7878/ */
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(now)) });
    let addr = ([127, 0, 0, 1], 7878).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}