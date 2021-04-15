fn main() {
    const MESSAGE: &str = "Hello, Rust!";
    println!("{}", MESSAGE);
}

#[cfg(test)]
mod tests {
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
            name:String,
            age:u32
        }
        let e = Person { name:String::from("Fred Flintstone"), age:27 };
        assert_eq!( e.name, "Fred Flintstone" );
        assert_eq!( e.age, 27 );
    }

    #[test]
    fn enumeration() {
        #[derive(PartialEq, Eq)]

        enum Gender {
            Male, Female
        }
        struct Worker {
            name:String,
            age:u32,
            gender:Gender
        }
        let m = Worker { name:String::from("Barney Rebel"), age:26, gender:Gender::Male };
        assert_eq!(m.name, "Barney Rebel" );
        assert_eq!(m.age, 26 );
        assert!( m.gender.eq(&Gender::Male) );

        let f = Worker { name:String::from("Betty Rebel"), age:26, gender:Gender::Female };
        assert_eq!(f.name, "Betty Rebel" );
        assert_eq!(f.age, 26 );
        assert!( f.gender.eq(&Gender::Female) );
    }

    #[test]
    fn generic() {
        struct Data<T> {
            value:T,
         }
        let i:Data<i32> = Data { value:3 };
        assert_eq!( i.value, 3 );

        let s:Data<String> = Data { value:"3".to_string() };
        assert_eq!( s.value, "3" );
    }

    #[test]
    fn traits() {
        struct Wrench {
            model:String,
            number:String
        }

        trait Part {
            fn id(&self) -> String;
        }

        impl Part for Wrench {
            fn id(&self) -> String {
                let mut m:String = String::from( &self.model );
                let n:String = String::from( &self.number );
                m.push_str(&n);
                m
            }
        }

        let w = Wrench { model:String::from("abc"), number:String::from("123") };
        assert_eq!( w.model, "abc" );
        assert_eq!( w.number, "123" );
        assert_eq!( w.id(), "abc123");
    }

    #[test]
    fn forloop() {
        let mut sum = 0;
        for i in 1..4 {
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
    fn loopy() {
        let mut sum = 0;
        loop {
            sum += 1;
            if sum == 6 {
               break;
            }
         }
        assert_eq!( sum, 6 );
    }
}