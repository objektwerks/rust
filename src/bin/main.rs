use log::info;
use log4rs;

use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("log4rs logger initialized!");

    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir();
    info!("args: {:?}", args);
    info!("current dir: {:?} ", current_dir.unwrap().display());

    let key = "key";
    let value = "value";
    info!("pre-set -> key: {} value: {}", key, value);
    env::set_var(key, value);
    info!("post-set -> key: {} value: {:?}", key, env::var(key));

    const MESSAGE: &str = "Hello, Rust!";
    println!("{}", MESSAGE);

    let handle = thread::spawn(|| {
        for i in 1..11 {
            println!("Spawned thread #{}!", i);
            thread::sleep(Duration::from_millis(i));
        }
        println!("{}", "Goodbye, Rust!");
    });

    for j in 1..11 {
        println!("Main thread #{}!", j);
        thread::sleep(Duration::from_millis(j));
    }

    handle.join().unwrap();
}