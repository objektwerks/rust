#[cfg(test)]
mod collections {
    #[test]
    fn iterator() {
        let a = [1, 2, 3];

        assert_eq!( a.iter().filter(|&x| x % 2 == 0).sum::<i32>(), 2 );
        assert_eq!( a.iter().fold(0, |x, y| x + y), 6 );
        assert_eq!( a.iter().map(|x| x * x).sum::<i32>(), 14 );
        assert!( a.iter().take(1).len() == 1 );
    }

    #[test]
    fn array() {
        let a = [1, 2, 3];
        assert_eq!( a.len(), 3 );
    }

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];
        assert_eq!( v.len(), 3 );
    }

    #[test]
    fn linkedlist() {
        use std::collections::LinkedList;

        let ll = LinkedList::from( [1, 2, 3] );
        assert_eq!( ll.front(), Some(&1) );
        assert_eq!( ll.back(), Some(&3) );
    }

    #[test]
    fn range() {
        let r = 1..4;
        assert_eq!( r.len(), 3 );
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
        assert!( hs.contains(&3) );
    }

    #[test]
    fn hashmap() {
        use std::collections::HashMap;
        
        let hm = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        assert_eq!( hm.get(&3), Some(&3) );
    }
}