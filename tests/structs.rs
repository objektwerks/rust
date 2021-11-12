#[cfg(test)]
mod structs {
    #[test]
    fn structure() {
        struct Person {
            name: String,
            age: u32,
        }
        let p = Person { name: String::from("Fred Flintstone"), age: 27 };
        assert_eq!(p.name, "Fred Flintstone");
        assert_eq!(p.age, 27);
    }
}