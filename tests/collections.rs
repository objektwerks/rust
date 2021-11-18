#[cfg(test)]
mod collections {
    #[test]
    fn array() {
        let a = [1, 2, 3];

        a.iter().for_each(|x| assert!( x > &0 ));
        assert_eq!( a.len(), 3 );
        assert_eq!( a.iter().count(), 3 );

        assert_eq!([1, 2, 3].iter().eq([1, 2, 3].iter()), true);
        assert_eq!([1].iter().ne([1, 2, 3].iter()), true);
        assert_eq!([1].iter().lt([1, 2, 3].iter()), true);
        assert_eq!([1, 2, 3].iter().gt([1].iter()), true);

        assert_eq!( a.iter().filter(|&x| x % 2 == 0).next().unwrap_or(&0), &2 );
        assert_eq!( a.iter().fold(0, |x, y| x + y), 6 );

        assert_eq!( a.iter().last().unwrap_or(&0), &3 );
        assert_eq!( a.iter().min().unwrap_or(&0), &1 );
        assert_eq!( a.iter().max().unwrap_or(&0), &3 );
        assert_eq!( a.iter().take(1).next().unwrap_or(&0), &1 );

        assert_eq!( a.iter().map(|x| x * x).last().unwrap_or(0), 9 );

        let b = ["1", "a", "2", "3"];
        let c = ["4", "b", "5", "6"];

        assert_eq!( b.iter().map(|s| s.parse::<u32>().ok()).flatten().sum::<u32>(), 6 );
        assert_eq!( b.iter().flat_map(|s| s.parse::<u32>().ok()).sum::<u32>(), 6 );
        assert_eq!( b.iter().chain(c.iter()).flat_map(|s| s.parse::<u32>().ok()).sum::<u32>(), 21 );

        assert_eq!( [1, 2, 3].iter().zip([4, 5, 6]).map(|(x, y)| x + y).sum::<u32>(), 21 );
        assert_eq!( a.into_iter().reduce(|x, y| x + y).unwrap_or(0), 6 );

        assert_eq!( a.to_vec(), vec![1, 2, 3] )
    }

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];
        assert_eq!( v.get(0).unwrap_or(&0), &1 );

        let mut mv = vec![];
        mv.push(1);
        mv.push(2);
        mv.push(3);
        assert_eq!( mv.first().unwrap_or(&0), &1 );
        assert_eq!( mv.last().unwrap_or(&0), &3 );
        assert_eq!( v, mv );
    }

    #[test]
    fn vecdeque() {
        use std::collections::VecDeque;

        let vd = VecDeque::from( [1, 2, 3] );
        assert_eq!( vd.get(0).unwrap_or(&0), &1 );

        let mut mvd = VecDeque::new();
        mvd.push_front(3);
        mvd.push_front(2);
        mvd.push_front(1);
        assert_eq!(mvd.front().unwrap_or(&0), &1 );
        assert_eq!(vd, mvd);
    }

    #[test]
    fn linkedlist() {
        use std::collections::LinkedList;

        let ll = LinkedList::from( [1, 2, 3] );
        assert_eq!( ll.front().unwrap_or(&0), &1 );
        assert_eq!( ll.back().unwrap_or(&0), &3 );

        let mut mll = LinkedList::new();
        mll.push_front(3);
        mll.push_front(2);
        mll.push_front(1);
        assert_eq!( mll.front().unwrap_or(&0), &1 );
        assert_eq!( mll.back().unwrap_or(&0), &3 );
    }

    #[test]
    fn range() {
        let r = 1..4;
        assert_eq!( r.len(), 3 );
    }

    #[test]
    fn string() {
        let s = String::from("string");
        assert_eq!( s.len(), 6 );
    }

    #[test]
    fn tuple() {
        let t:(u32, u32, u32) = (1, 2, 3);
        assert_eq!( t.0 + t.1 + t.2, 6 );
    }

    #[test]
    fn hashset() {
        use std::collections::HashSet;

        let hs = HashSet::from([1, 2, 3]);
        assert!( hs.contains(&3) );
    }

    #[test]
    fn btreeset() {
        use std::collections::BTreeSet;

        let hs = BTreeSet::from([1, 2, 3]);
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

    #[test]
    fn binaryheap() {
        use std::collections::BinaryHeap;

        let bh = BinaryHeap::from( [1, 2, 3] );
        assert_eq!( bh.peek().unwrap_or(&0), &3 );
    }
}