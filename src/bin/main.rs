fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    println!("main args: {:?}", args);

    const MESSAGE: &str = "Hello, Rust!";
    println!("{}", MESSAGE);

    use std::thread;
    use std::time::Duration;

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