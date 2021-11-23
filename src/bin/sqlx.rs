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
    async fn update(&self, pool: &Pool<Postgres>) -> anyhow::Result<u64> {
        let rows_affected = sqlx::query!(
            r#"UPDATE todo SET completed = $1 WHERE id = $2"#,
            self.completed,
            self.id
        )
        .execute(pool)
        .await?
        .rows_affected();
        Ok(rows_affected)
    }
    async fn select(pool: &Pool<Postgres>) -> anyhow::Result<Vec<Todo>> {
        let todos = sqlx::query_as!(
            Todo,
            "SELECT id, task, started, completed FROM todo"
        )
        .fetch_all(pool)
        .await?;
        Ok(todos)
    }
    async fn delete(&self, pool: &Pool<Postgres>) -> anyhow::Result<u64> {
        let result: PgQueryResult = sqlx::query!(
            r#"DELETE from todo WHERE id = $1"#,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }
}

use sqlx::postgres::{PgPoolOptions, PgQueryResult};

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

    // insert
    let mut todo = Todo::new(1, "wash car");
    todo.id = Todo::insert(&todo, &pool).await.unwrap();
    println!("inserted todo: {:?}", todo);

    // update
    todo.completed = Local::now().to_string();
    let updated = Todo::update( &todo, &pool ).await.unwrap();
    println!("updated {} todo: {:?}", updated, todo);

    // select
    let todos = Todo::select( &pool ).await.unwrap();
    for todo in todos {
        println!("selected todo: {:?}", todo);
    }

    // delete
    let deleted = Todo::delete( &todo, &pool ).await.unwrap();
    println!("deleted {} todo: {:?}", deleted, todo);

    Ok(())
}