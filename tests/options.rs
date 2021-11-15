#[cfg(test)]
mod options {
    #[test]
    fn option() {
        fn to_int(s: &str) -> Option<i32> {
            return s.parse::<i32>().ok();
        }

        assert!( to_int("3").is_some() );
        assert!( to_int("c").is_none() );

        assert_eq!( to_int("3").unwrap_or(0), 3 );
        assert_eq!( to_int("c").unwrap_or(0), 0 );

        let ns = ["1", "a", "2", "3"];

        assert_eq!( ns.iter().map(|s| to_int(s)).flatten().sum::<i32>(), 6 );
        assert_eq!( ns.iter().flat_map(|s| to_int(s)).sum::<i32>(), 6 );
    }
}