#[cfg(test)]
mod borrowing {
    #[test]
    fn reference() {
        fn calc_len(string: &String) -> usize { // borrower
            string.len()
        }

        let value = String::from("value"); // owner
        let len = calc_len( &value ); // borrower

        assert_eq!( len, 5 );
    }
}