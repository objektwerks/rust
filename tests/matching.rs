#[cfg(test)]
mod matching {
    #[test]
    fn matching() {
        fn square(i: i32) -> i32 {
            match i {
                1 => 1 * 1,
                2 => 2 * 2,
                3 => 3 * 3,
                _ => 0
            }
        }
        assert_eq!( square(1), 1 );
        assert_eq!( square(2), 4 );
        assert_eq!( square(3), 9 );
        assert_eq!( square(4), 0 );
    }
}