use chrono::Local;

#[derive(Debug)]
struct Todo {
    id: i32,
    task: String,
    started: String,
    completed: String,
}

impl Todo {
    fn new(id: i32, task: &str) -> Todo {
        Todo {
            id,
            task: task.to_string(),
            started: Local::now().to_string(),
            completed: "".to_string(),
        }
    }

    async fn insert(&self, pool: &Pool<Postgres>) -> anyhow::Result<i32> {
        let row = sqlx::query!(
            r#"INSERT INTO todo (task, started, completed) VALUES ($1, $2, $3) RETURNING id"#,
            self.task,
            self.started,
            self.completed
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }
}

use sqlx::postgres::PgPoolOptions;

use std::env;
use sqlx::{Pool, Postgres};

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(url.as_str())
        .await?;
    println!("pool: {:?}", pool);

    let mut todo = Todo::new(1, "wash car");
    let future_id = Todo::insert(&todo, &pool).await;
    todo.id = future_id.unwrap();
    println!("todo: {:?}, ", todo);

    Ok(())
}