#[cfg(test)]
mod traits {
    #[test]
    fn traits() {
        struct Wrench {
            model: String,
            number: String,
        }
        impl Wrench {
            fn new(model: String, number: String) -> Wrench {
                Wrench { model, number }
            }
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

        let w = Wrench::new(String::from("abc"), String::from("123"));
        assert_eq!(w.model, "abc");
        assert_eq!(w.number, "123");
        assert_eq!(w.id(), w.model + &*w.number);
    }
}