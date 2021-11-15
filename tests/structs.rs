#[cfg(test)]
mod structs {
    #[test]
    fn structure() {
        struct Person {
            first: String,
            last: String,
            age: u32,
        }
        impl Person {
            fn name(&self) -> String {
                return self.first.to_string() + " " + &self.last.to_string();
            }
        }

        let p = Person { first: "Fred".to_string(), last: "Flintstone".to_string(), age: 27 };
        assert_eq!( p.first, "Fred" );
        assert_eq!( p.last, "Flintstone" );
        assert_eq!( p.age, 27 );
        assert_eq!( p.name(), "Fred Flintstone" )
    }
}