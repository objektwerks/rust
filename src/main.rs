fn main() {
    const MESSAGE: &str = "Hello, Rust!";
    println!("{}", MESSAGE);
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn array() {
        let mut a = [1, 2, 3];
        a.reverse();
        assert_eq!( a, [3, 2, 1] );
    }

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];
        assert_eq!( v[2], 3 );
    }

    #[test]
    fn tuple() {
        let t = (1, 2, 3);
        assert_eq!( t.2, 3 );
    }

    #[test]
    fn hashset() {
        use std::collections::HashSet;

        let mut hs = HashSet::new();
        hs.insert(1);
        hs.insert(2);
        hs.insert(3);
        assert!( hs.contains(&3) );
    }

    #[test]
    fn hashmap() {
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        hm.insert(1, 1);
        hm.insert(2, 2);
        hm.insert(3, 3);
        assert_eq!( hm.get(&3), Some(&3) );
    }

    #[test]
    fn structure() {
        struct Person {
            name: String,
            age: u32
        }
        let p = Person { name: String::from("Fred Flintstone"), age: 27 };
        assert_eq!( p.name, "Fred Flintstone" );
        assert_eq!( p.age, 27 );
    }

    #[test]
    fn enumeration() {
        #[derive(PartialEq, Eq)]

        enum Gender {
            Male, Female
        }
        struct Worker {
            name: String,
            age: u32,
            gender: Gender
        }
        let m = Worker { name: String::from("Barney Rebel"), age: 26, gender: Gender::Male };
        assert_eq!( m.name, "Barney Rebel" );
        assert_eq!( m.age, 26 );
        assert!( m.gender.eq(&Gender::Male) );

        let f = Worker { name: String::from("Betty Rebel"), age: 26, gender: Gender::Female };
        assert_eq!( f.name, "Betty Rebel" );
        assert_eq!( f.age, 26 );
        assert!( f.gender.eq(&Gender::Female) );
    }

    #[test]
    fn generic() {
        struct Data<T> {
            value: T
         }
        let i: Data<i32> = Data { value: 3 };
        assert_eq!( i.value, 3 );

        let s: Data<String> = Data { value: "3".to_string() };
        assert_eq!( s.value, "3" );
    }

    #[test]
    fn traits() {
        struct Wrench {
            model: String,
            number: String
        }

        trait Part {
            fn id(&self) -> String;
        }

        impl Part for Wrench {
            fn id(&self) -> String {
                let m: String = String::from( &self.model );
                let n: String = String::from( &self.number );
                m + &n
            }
        }

        let w = Wrench { model: String::from("abc"), number: String::from("123") };
        assert_eq!( w.model, "abc" );
        assert_eq!( w.number, "123" );
        assert_eq!( w.id(), "abc123" );
    }

    #[test]
    fn pattern() {
        fn square(i: i32) -> i32 {
            let v = match i {
                1 => 1 * 1,
                2 => 2 * 2,
                3 => 3 * 3,
                _ => 0
            };
            v
        }
        assert_eq!(square(1), 1);
        assert_eq!(square(2), 4);
        assert_eq!(square(3), 9);
        assert_eq!(square(4), 0);
    }

    #[test]
    fn forloop() {
        let mut sum = 0;
        for i in 1..4 {
            sum += i
        }
        assert_eq!( sum, 6 );

        let a = [1, 2, 3];
        let iter = a.iter();
        sum = 0;
        for i in iter {
            sum += i
        }
        assert_eq!( sum, 6 );
    }

    #[test]
    fn whileloop() {
        let mut sum = 0;
        while sum < 6 {
            sum += 1
         }
        assert_eq!( sum, 6 );
    }

    #[test]
    fn loopbreak() {
        let mut sum = 0;
        loop {
            sum += 1;
            if sum == 6 {
               break;
            }
         }
        assert_eq!( sum, 6 );
    }

    #[test]
    fn forcontinue() {
        let mut sum = 0;
        for i in 1..4 {
            if i % 2 == 0 {
               continue;
            }
            sum += i;
         }
        assert_eq!( sum, 4 );
    }

    #[test]
    fn string() {
        let mut s = String::new();
        s.push_str("fred ");
        s.push_str("flintstone");
        assert_eq!( s, "fred flintstone" );

        let b = "barney ".to_string();
        let r = "rebel".to_string();
        let br = b + &r;
        assert_eq!( br, "barney rebel" );

        let i = 3;
        let is = i.to_string();
        assert_eq!( is, "3" );

        let w = "wilma".to_string();
        let f = "flintstone".to_string();
        let wf = format!("{} {}", w, f);
        assert_eq!( wf, "wilma flintstone" );

        let model: String = String::from("abc");
        let number: String = String::from("123");
        let id: String = model + &number;
        assert_eq!( id, "abc123" );
    }

    #[test]
    fn result() {
        use std::fs::File;

        let err = File::open("crate.toml");
        assert!( err.is_err() );

        let ok = File::open("Cargo.toml");
        assert!( ok.is_ok() );

        let mut file = ok.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert!( !contents.is_empty() );
    }
}