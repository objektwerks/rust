#[macro_use] extern crate rocket;

use chrono::Utc;

#[get("/now")]
fn now() -> String {
    Utc::now().to_string()
}

/** To test server: curl http://localhost:8080/ */
#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount("/", routes![now])
        .launch()
        .await {
            println!("Rocket failed launch!");
            drop(e);
        };
}