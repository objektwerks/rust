#[cfg(test)]
mod options {
    #[test]
    fn option() {
        fn to_int(s: &str) -> Option<u32> {
            return s.parse::<u32>().ok();
        }

        assert!( to_int("3").is_some() );
        assert!( to_int("c").is_none() );

        assert_eq!( to_int("3").unwrap_or(0), 3 );
        assert_eq!( to_int("c").unwrap_or(0), 0 );

        match to_int("3") {
            Some(i) => assert_eq!(i, 3),
            None => panic!("Should be Some(3)"),
        }
        match to_int("c") {
            Some(_) => panic!("Should be None"),
            None => assert!(true),
        }

        let ns = ["1", "a", "2", "3"];

        assert_eq!( ns.iter().map(|s| to_int(s)).flatten().sum::<u32>(), 6 );
        assert_eq!( ns.iter().flat_map(|s| to_int(s)).sum::<u32>(), 6 );
    }
}