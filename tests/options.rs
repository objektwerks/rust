#[cfg(test)]
mod options {
    #[test]
    fn option() {
        fn to_int(s: &str) -> Option<i32> {
            let i: Option<i32> = s.parse::<i32>().ok();
            i
        }
        assert_eq!(to_int("3").unwrap_or(0), 3);
        assert_eq!(to_int("c").unwrap_or(0), 0);

        let b = ["1", "a", "2", "3"];

        assert_eq!( b.iter().map(|s| to_int(s)).flatten().sum::<i32>(), 6 );
        assert_eq!( b.iter().flat_map(|s| to_int(s)).sum::<i32>(), 6 );
    }
}