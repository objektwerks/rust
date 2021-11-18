#[cfg(test)]
mod concurrency {
    #[test]
    fn thread() {
        use std::thread;
        use std::time::Duration;

        let mut i_count = 0;
        let handle = thread::spawn( move || { // move ownership of i_count to thread closure!
            for i in 1..11 {
                assert_eq!( i, i );
                i_count = i_count + 1;
                thread::sleep(Duration::from_millis(i));
            }
            assert_eq!( i_count, 10 );
        });

        let mut j_count = 0;
        for j in 1..11 {
            assert_eq!( j, j );
            j_count = j_count + 1;
            thread::sleep(Duration::from_millis(j));
        }
        assert_eq!( j_count, 10 );

        handle.join().unwrap();
    }

    #[test]
    fn channel() {
        use std::sync::mpsc;
        use std::thread;

        let (producer, consumer) = mpsc::channel();

        thread::spawn(move || {
            let message = 1;
            producer.send(message).unwrap();
        });

        let message = consumer.recv().unwrap();
        assert_eq!(message, 1 )
    }
}