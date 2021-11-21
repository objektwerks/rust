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

fn main() {
    println!("todo: {:?}, ", Todo::new(1, "wash car"));
}