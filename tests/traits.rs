#[cfg(test)]
mod traits {
    #[test]
    fn traits() {
        struct Wrench {
            model: String,
            number: String,
        }

        trait Part {
            fn id(&self) -> String;
        }

        impl Part for Wrench {
            fn id(&self) -> String {
                let m: String = String::from(&self.model);
                let n: String = String::from(&self.number);
                m + &n
            }
        }

        let w = Wrench { model: String::from("abc"), number: String::from("123") };
        assert_eq!(w.model, "abc");
        assert_eq!(w.number, "123");
        assert_eq!(w.id(), "abc123");
    }
}