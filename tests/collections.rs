#[cfg(test)]
mod collections {
    #[test]
    fn array() {
        let a = [1, 2, 3];
        assert_eq!( a.iter().fold(0, |x, y| x + y), 6 );
        assert_eq!( a.iter().map(|x| x * x).sum::<i32>(), 14)
    }

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];
        assert_eq!( v.iter().fold(0, |x, y| x + y), 6 );
        assert_eq!( v.iter().map(|x| x * x).sum::<i32>(), 14)
    }

    #[test]
    fn tuple() {
        let t:(i32, i32, i32) = (1, 2, 3);
        assert_eq!(t.0 + t.1 + t.2, 6);
    }

    #[test]
    fn hashset() {
        use std::collections::HashSet;

        let mut hs = HashSet::new();
        hs.insert(1);
        hs.insert(2);
        hs.insert(3);
        assert!(hs.contains(&3));
        assert_eq!( hs.iter().fold(0, |x, y| x + y), 6 );
    }

    #[test]
    fn hashmap() {
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        hm.insert(1, 1);
        hm.insert(2, 2);
        hm.insert(3, 3);
        assert_eq!(hm.get(&3), Some(&3));
        assert_eq!( hm.into_values().fold(0, |x, y| x + y), 6 );
    }
}