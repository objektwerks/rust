#[cfg(test)]
mod borrowing {
    /*
    1. At any given time, you can have either one mutable
       reference or any number of immutable references.
    2. References must always be valid.
    */

    #[test]
    fn ref_immutable() {
        fn calc_len(string: &String) -> usize { // borrower
            string.len()
        }

        let value = String::from("value"); // owner
        let len = calc_len( &value );

        assert_eq!( len, 5 );
    }

    #[test]
    fn ref_mutable() {
        fn mutate(string: &mut String) { // borrower
            string.push_str(", value");
        }

        let mut value = String::from("value"); // owner
        mutate( &mut value );

        assert_eq!( value, "value, value".to_string() );
    }
}