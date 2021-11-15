#[cfg(test)]
mod matching {
    #[test]
    fn matching() {
        fn square(i: i32) -> i32 {
            match i {
                1 => 1 * 1,
                n => n * n,
            }
        }
        assert_eq!( square(1), 1 );
        assert_eq!( square(4), 16 );
    }
}