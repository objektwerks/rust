use chrono::Utc;

use tide::Request;
use tide::Result;

async fn now(_: Request<()>) -> Result {
    Ok(format!("Now: {}", Utc::now().to_string()).into())
}

/**
To test server: curl http://localhost:8081/
*/
#[async_std::main]
async fn main() -> Result<()> {
    let mut server = tide::new();
    server.at("/now").get(now);
    server.listen("127.0.0.1:8081").await?;
    Ok(())
}