use chrono::Local;

#[derive(Debug)]
struct Todo {
    id: u32,
    task: String,
    started: String,
    completed: String,
}

impl Todo {
    fn new(id: u32, task: &str) -> Todo {
        Todo {
            id,
            task: task.to_string(),
            started: Local::now().to_string(),
            completed: "".to_string(),
        }
    }
}

use sqlx::postgres::PgPoolOptions;

use std::env;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = env::var("TODO_POSTGRESQL_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(url.as_str())
        .await?;

    println!("pool: {:?}", pool);
    println!("todo: {:?}, ", Todo::new(1, "wash car"));

    Ok(())
}