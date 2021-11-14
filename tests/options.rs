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
    }
}