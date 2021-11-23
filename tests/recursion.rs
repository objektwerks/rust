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
        assert_eq!( factorial(9, 1), 362880 )
    }

    #[test]
    fn fibonacci() {
        fn fibonacci(n: i32, a: i32, b: i32) -> i32 {
            match n {
                0 => a,
                _ => fibonacci(n - 1, b, a + b),
            }
        }
        assert_eq!( fibonacci(9, 0, 1), 34 );
    }
}