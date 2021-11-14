#[cfg(test)]
mod collections {
    #[test]
    fn iterator() {
        let a = [1, 2, 3];

        a.iter().for_each(|x| assert!( x > &0 ));
        assert_eq!( a.len(), 3 );
        assert_eq!( a.iter().count(), 3 );
        assert_eq!( a.iter().filter(|&x| x % 2 == 0).next().unwrap_or(&0), &2 );
        assert_eq!( a.iter().fold(0, |x, y| x + y), 6 );
        assert_eq!( a.iter().last().unwrap_or(&0), &3 );
        assert_eq!( a.iter().min().unwrap_or(&0), &1 );
        assert_eq!( a.iter().max().unwrap_or(&0), &3 );
        assert_eq!( a.iter().map(|x| x * x).last().unwrap_or(0), 9 );
        assert_eq!( a.iter().take(1).next().unwrap_or(&0), &1 );

        let b = ["1", "a", "2", "3"];
        let c = ["4", "b", "5", "6"];

        assert_eq!( b.iter().map(|s| s.parse::<i32>().ok()).flatten().sum::<i32>(), 6 );
        assert_eq!( b.iter().flat_map(|s| s.parse::<i32>().ok()).sum::<i32>(), 6 );
        assert_eq!( b.iter().chain(c.iter()).flat_map(|s| s.parse::<i32>().ok()).sum::<i32>(), 21 );
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

    #[test]
    fn btreemap() {
        use std::collections::BTreeMap;

        let btm = BTreeMap::from([(1, 1), (2, 2), (3, 3)]);
        assert_eq!( btm.get(&3), Some(&3) );
    }
}