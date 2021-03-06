use chrono::Local;

use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Todo {
    id: u32,
    task: String,
    started: String,
    completed: String,
}

impl Todo {
    const INSERTED: &'static str = "inserted";
    const UPDATED: &'static str = "updated";
    const SELECTED: &'static str = "selected";
    const DELETED: &'static str = "deleted";

    fn new(id: u32, task: &str) -> Todo {
        Todo {
            id,
            task: task.to_string(),
            started: Local::now().to_string(),
            completed: "".to_string(),
        }
    }
    fn create_table(connection: &Connection) -> Result<usize> {
        connection.execute(
            "CREATE TABLE todo (
             id        INTEGER PRIMARY KEY,
             task      TEXT NOT NULL,
             started   TEXT NOT NULL,
             completed TEXT NOT NULL
             )",
             [],
        )
    }
    fn insert(&self, connection: &Connection) -> Result<usize> {
        connection.execute(
            "INSERT INTO todo (task, started, completed) VALUES (?1, ?2, ?3)",
            params![ self.task, self.started, self.completed ],
        )
    }
    fn update(&self, connection: &Connection) -> Result<usize> {
        connection.execute(
            "UPDATE todo SET completed = ?1 WHERE id = ?2",
            params![ self.completed, self.id ],
        )
    }
    fn select(connection: &Connection) -> Result<Vec<Todo>> {
        let mut statement = connection.prepare("SELECT id, task, started, completed FROM todo")?;
        let result = statement.query_map([], |row| {
            Ok(
                Todo {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    started: row.get(2)?,
                    completed: row.get(3)?,
                }
            )
        })?;
        let mut todos = Vec::new();
        for item in result {
            todos.push(item?);
        }
        Ok(todos)
    }
    fn delete(&self, connection: &Connection) -> Result<usize> {
        connection.execute(
            "DELETE from todo WHERE id = ?1",
            params![ self.id ],
        )
    }
    fn println(&self, message: &str, rows: usize) -> () {
        println!("{}: [{}] {:?}", message, rows, self);
    }
    fn printlns(message: &str, result: Result<Vec<Todo>>) -> () {
        let mut count = 1;
        for todo in result.unwrap() {
            println!("{}: [{}] {:?}", message, count, todo);
            count += 1;
        }
        if count == 1 {
            println!("selected is empty!");
        }
    }
}

fn main() -> Result<()> {
    // connect
    let connection = Connection::open_in_memory()?;
    Todo::create_table( &connection )?;

    // insert
    let mut todo = Todo::new( 1, "mow yard" );
    let inserted = Todo::insert( &todo, &connection )?;
    Todo::println( &todo, Todo::INSERTED, inserted );

    // select
    Todo::printlns( Todo::SELECTED, Todo::select( &connection ) );

    // update
    todo.completed = Local::now().to_string();
    let updated = Todo::update( &todo, &connection )?;
    Todo::println( &todo, Todo::UPDATED, updated );

    // select
    Todo::printlns( Todo::SELECTED, Todo::select( &connection ) );

    // delete
    let deleted = Todo::delete( &todo, &connection )?;
    Todo::println( &todo, Todo::DELETED, deleted );

    // select
    Todo::printlns( Todo::SELECTED, Todo::select( &connection ) );

    Ok(())
}