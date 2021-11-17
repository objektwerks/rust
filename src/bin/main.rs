fn main() {
    const MESSAGE: &str = "Hello, Rust!";
    println!("{}", MESSAGE);

    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..11 {
            println!("I'm thread #{}!", i);
            thread::sleep(Duration::from_millis(i));
        }
        println!("{}", "Goodbye, Rust!");
    });
    handle.join().unwrap();
}