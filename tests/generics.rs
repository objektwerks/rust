#[cfg(test)]
mod generics {
    #[test]
    fn generics() {
        struct Data<T> {
            value: T,
        }

        let i: Data<u32> = Data { value: 3 };
        assert_eq!(i.value, 3);

        let s: Data<String> = Data { value: "3".to_string() };
        assert_eq!(s.value, "3");
    }
}