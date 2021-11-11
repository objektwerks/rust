use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use chrono::Utc;

#[get("/now")]
async fn now() -> impl Responder {
    HttpResponse::Ok().body(Utc::now().to_string())
}

/** To test server: curl http://localhost:7777/ */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(now))
        .bind("127.0.0.1:7777")?
        .run()
        .await
}
