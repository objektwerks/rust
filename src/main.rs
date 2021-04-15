fn main() {
    let rust = "Hello, Rust!";
    println!("{}", rust);
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
}