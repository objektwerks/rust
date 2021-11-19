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
    fn new(id: u32, task: String) -> Todo {
        Todo {
            id,
            task,
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
        let mut select_todos = connection.prepare("SELECT id, task, started, completed FROM todo")?;
        let todos = select_todos.query_map([], |row| {
            Ok(
                Todo {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    started: row.get(2)?,
                    completed: row.get(3)?,
                }
            )
        })?;
        let mut list = Vec::new();
        for todo in todos {
            list.push(todo?);
        }
        Ok(list)
    }
    fn delete(&self, connection: &Connection) -> Result<usize> {
        connection.execute(
            "DELETE from todo WHERE id = ?1",
            params![ self.id ],
        )
    }
    fn print_insert(&self, rows: usize) -> () {
        println!("inserted: [{}] {:?}", rows, self);
    }
    fn print_update(&self, rows: usize) -> () {
        println!("updated: [{}] {:?}", rows, self);
    }
    fn print_delete(&self, rows: usize) -> () {
        println!("deleted: [{}] id -> {:?}", rows, self.id);
    }
    fn print_select(todos: Result<Vec<Todo>>) -> () {
        let mut count = 1;
        for todo in todos.unwrap() {
            println!("selected: [{}] {:?}", count, todo);
            count += 1;
        }
        if count == 1 {
            println!("selected is empty!");
        }
    }
}

fn main() -> Result<()> {
    let connection = Connection::open_in_memory()?;
    Todo::create_table( &connection )?;

    let mut todo = Todo::new( 1, "mow yard".to_string() );
    let inserted = Todo::insert(&todo, &connection )?;
    Todo::print_insert(&todo, inserted);

    Todo::print_select( Todo::select(&connection) );

    todo.completed = Local::now().to_string();
    let updated = Todo::update(&todo, &connection )?;
    Todo::print_update(&todo, updated);

    Todo::print_select( Todo::select(&connection) );

    let deleted = Todo::delete(&todo, &connection)?;
    Todo::print_delete(&todo, deleted);

    Todo::print_select( Todo::select(&connection) );

    Ok(())
}