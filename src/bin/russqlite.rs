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
    fn print_insert(&self, rows: usize) -> () {
        println!("inserted: [{}] {:?}", rows, self);
    }
    fn print_select(todos: Result<Vec<Todo>>) -> () {
        let mut count = 1;
        for todo in todos.unwrap() {
            println!("selected: [{}] {:?}", count, todo);
            count += 1;
        }
    }
}

fn main() -> Result<()> {
    let connection = Connection::open_in_memory()?;
    Todo::table( &connection )?;

    let todo = Todo::new( "mow yard".to_string() );
    let rows = Todo::insert( &todo, &connection )?;
    Todo::print_insert(&todo, rows);

    Todo::print_select( Todo::select(&connection) );

    Ok(())
}