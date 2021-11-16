#[cfg(test)]
mod functions {
    #[test]
    fn higher_order() {
        fn square(i: i32) -> i32 {
            return i * i;
        }

        println!("Squared {} to {}", 3, square(3));
    }
}