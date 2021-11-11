#[macro_use]
extern crate rocket;

use chrono::Utc;

#[get("/now")]
fn now() -> &'static str {
    Utc::now().to_string().as_str()
}

/** To test server: curl http://localhost:8080/ */
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![now])
        .launch()
        .await
}