#[cfg(test)]
mod enums {
    #[test]
    fn enums() {
        #[derive(PartialEq, Eq)]
        enum Gender {
            Male,
            Female,
        }

        struct Worker {
            name: String,
            age: u32,
            gender: Gender,
        }
        impl Worker {
            fn new(name: String, age: u32, gender: Gender) -> Worker {
                Worker { name, age, gender }
            }
        }

        let m = Worker::new(String::from("Barney Rebel"), 26, Gender::Male);
        assert_eq!(m.name, "Barney Rebel");
        assert_eq!(m.age, 26);
        assert!(m.gender.eq(&Gender::Male));

        let f = Worker::new(String::from("Betty Rebel"), 26, Gender::Female);
        assert_eq!(f.name, "Betty Rebel");
        assert_eq!(f.age, 26);
        assert!(f.gender.eq(&Gender::Female));
    }
}