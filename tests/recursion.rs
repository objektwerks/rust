#[cfg(test)]
mod recursion {
    #[test]
    fn factorial() {
        fn factorial(n: u32, acc: u32) -> u32 {
            if n == 1 {
                return acc;
            } else {
                factorial(n - 1, acc * n)
            }
        }
        assert_eq!(factorial(9, 1), 362880)
    }
}