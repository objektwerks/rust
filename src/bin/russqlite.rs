use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Todo {
    id: i32,
    task: String,
}

impl Todo {
    fn new(task: String) -> Todo {
        Todo { id: 0, task }
    }
    fn table(connection: &Connection) -> Result<usize> {
        connection.execute(
            "CREATE TABLE todo (
             id   INTEGER PRIMARY KEY AUTOINCREMENT,
             task TEXT NOT NULL
         )",
            [],
        )
    }
    fn insert(&self, connection: &Connection) -> Result<usize> {
        connection.execute(
            "INSERT INTO todo (task) VALUES (?1)",
            params![ self.task ],
        )
    }
    fn select(connection: &Connection) -> Result<Vec<Todo>> {
        let mut select_todos = connection.prepare("SELECT id, task FROM todo")?;
        let todos = select_todos.query_map([], |row| {
            Ok(
                Todo {
                    id: row.get(0)?,
                    task: row.get(1)?,
                }
            )
        })?;
        let mut list = Vec::new();
        for todo in todos {
            list.push(todo?);
        }
        Ok(list)
    }
    fn debug(&self, rows: usize) -> () {
        println!("todo: [{}] {:?}", rows, self);
    }
    fn print(todos: Result<Vec<Todo>>) -> () {
        for todo in todos.unwrap() {
            println!("todo: {:?}", todo);
        }
    }
}

fn main() -> Result<()> {
    let connection = Connection::open_in_memory()?;
    Todo::table( &connection )?;

    let todo = Todo::new( "mow yard".to_string() );
    let rows = Todo::insert( &todo, &connection )?;
    Todo::debug(&todo, rows);

    Todo::print( Todo::select(&connection) );

    Ok(())
}