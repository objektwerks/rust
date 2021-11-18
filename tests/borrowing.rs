#[cfg(test)]
mod borrowing {
    #[test]
    fn reference() {
        fn calc_len(string: &String) -> usize {
            string.len()
        }

        let value = String::from("value");
        let len = calc_len( &value);

        assert_eq!( len, 5 );
    }
}