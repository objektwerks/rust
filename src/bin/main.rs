fn main() {
    const MESSAGE: &str = "Hello, Rust!";
    println!("{}", MESSAGE);

    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..4 {
            println!("Hi! I'm thread #{}!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
}