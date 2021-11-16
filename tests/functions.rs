#[cfg(test)]
mod functions {
    #[test]
    fn higher_order() {
        fn square(i: i32) -> i32 {
            return i * i;
        }

        let a = [1, 2, 3];
        assert_eq!( a.iter().map(|x| square(*x)).last().unwrap_or(0), 9 );
    }
}