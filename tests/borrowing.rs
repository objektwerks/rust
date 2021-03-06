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

    #[test]
    fn ref_dangling() {
        /*
        fn dangle() -> &String { // returns dangling reference
            return &String::from("value");
        }
        */

        fn undangle() -> String { // returns new String
            return String::from("value");
        }

        // let reference_to_nothing = dangle();
        let value = undangle();
        assert_eq!( value, "value".to_string() );
    }

    #[test]
    fn lifetime() {
        fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string_x = "three";
        let string_y = "four";
        assert_eq!( longest_str(string_x, string_y), string_x );

        struct PartHolder<'a> {
            part: &'a str,
        }

        let part_a = "part-a";
        let part_holder = PartHolder {
            part: part_a,
        };
        assert_eq!( part_holder.part.len(), 6 );
    }
}