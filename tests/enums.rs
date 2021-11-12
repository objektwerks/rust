#[cfg(test)]
mod enums {
    #[test]
    fn enumeration() {
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
        let m = Worker { name: String::from("Barney Rebel"), age: 26, gender: Gender::Male };
        assert_eq!(m.name, "Barney Rebel");
        assert_eq!(m.age, 26);
        assert!(m.gender.eq(&Gender::Male));

        let f = Worker { name: String::from("Betty Rebel"), age: 26, gender: Gender::Female };
        assert_eq!(f.name, "Betty Rebel");
        assert_eq!(f.age, 26);
        assert!(f.gender.eq(&Gender::Female));
    }
}