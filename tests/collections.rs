#[cfg(test)]
mod collections {
    #[test]
    fn array() {
        let a = [1, 2, 3];
        assert_eq!( a.iter().fold(0, |x, y| x + y), 6 );
        assert_eq!( a.iter().map(|x| x * x).sum::<i32>(), 14 );
        assert_eq!( a.iter().filter(|&x| x % 2 == 0).sum::<i32>(), 2 );
    }

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];
        assert_eq!( v.iter().fold(0, |x, y| x + y), 6 );
        assert_eq!( v.iter().map(|x| x * x).sum::<i32>(), 14 );
        assert_eq!( v.iter().filter(|&x| x % 2 == 0).sum::<i32>(), 2 );
    }

    #[test]
    fn tuple() {
        let t:(i32, i32, i32) = (1, 2, 3);
        assert_eq!( t.0 + t.1 + t.2, 6 );
    }

    #[test]
    fn hashset() {
        use std::collections::HashSet;

        let hs = HashSet::from([1, 2, 3]);
        assert!(hs.contains(&3));
        assert_eq!( hs.iter().fold(0, |x, y| x + y), 6 );
        assert_eq!( hs.iter().map(|x| x * x).sum::<i32>(), 14 );
        assert_eq!( hs.iter().filter(|&x| x % 2 == 0).sum::<i32>(), 2 );
    }

    #[test]
    fn hashmap() {
        use std::collections::HashMap;
        
        let hm = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        assert_eq!( hm.get(&3), Some(&3) );
        assert_eq!( hm.keys().fold(0, |x, y| x + y), 6 );
        assert_eq!( hm.values().fold(0, |x, y| x + y), 6 );
        assert_eq!( hm.keys().map(|x| x * x).sum::<i32>(), 14 ) ;
        assert_eq!( hm.values().map(|x| x * x).sum::<i32>(), 14 ) ;
        assert_eq!( hm.keys().filter(|&x| x % 2 == 0).sum::<i32>(), 2 );
        assert_eq!( hm.values().filter(|&x| x % 2 == 0).sum::<i32>(), 2 );
    }
}