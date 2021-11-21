use log::info;
use log4rs;

use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("log4rs logger initialized!");

    let args: Vec<String> = env::args().collect();
    println!("main args: {:?}", args);
    info!("logger args: {:?} ", args);

    const MESSAGE: &str = "Hello, Rust!";
    println!("{}", MESSAGE);
    info!("logger message: {:?} ", MESSAGE);

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