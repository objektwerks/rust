#[cfg(test)]
mod structs {
    #[test]
    fn structs() {
        struct Person {
            first: String,
            last: String,
            age: u32,
        }
        impl Person {
            fn new(first: String, last: String, age: u32) -> Person {
                Person { first, last, age }
            }
            fn name(&self) -> String {
                return self.first.to_string() + " " + &self.last.to_string();
            }
        }

        let p = Person::new("Fred".to_string(), "Flintstone".to_string(), 27);
        assert_eq!( p.first, "Fred" );
        assert_eq!( p.last, "Flintstone" );
        assert_eq!( p.age, 27 );
        assert_eq!( p.name(), "Fred Flintstone" )
    }

    #[test]
    fn tuple_structs() {
        #[derive(PartialEq, Debug)]
        struct Color(i32, i32, i32);

        const NAVY: Color = Color(0, 0, 128);
        assert_eq!( NAVY, Color(0, 0, 128) )
    }
}