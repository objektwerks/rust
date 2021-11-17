#[cfg(test)]
mod concurrency {
    #[test]
    fn thread() {
        use std::thread;
        use std::time::Duration;

        let handle = thread::spawn(|| {
            for i in 1..11 {
                assert_eq!( i, i );
                thread::sleep(Duration::from_millis(i));
            }
        });
        handle.join().unwrap();
    }
}