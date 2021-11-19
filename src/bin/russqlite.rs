use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Todo {
    id: i32,
    task: String,
}

fn main() -> Result<()> {
    // connect
    let connection = Connection::open_in_memory()?;
    connection.execute(
        "CREATE TABLE todo (
             id   INTEGER PRIMARY KEY,
             task TEXT NOT NULL
         )",
        [],
    )?;

    // insert
    let todo = Todo {
        id: 1,
        task: "mow yard".to_string(),
    };
    let rows = connection.execute(
        "INSERT INTO todo (task) VALUES (?1)",
        params![ todo.task ],
    )?;
    println!("inserted: {} {:?}", rows, todo);

    // select
    let mut select_todos = connection.prepare("SELECT id, task FROM todo")?;
    let todos = select_todos.query_map([], |row| {
        Ok(
            Todo {
                id: row.get(0)?,
                task: row.get(1)?,
            }
        )
    })?;
    for todo in todos {
        println!("selected: {:?}", todo.unwrap());
    }

    Ok(())
}