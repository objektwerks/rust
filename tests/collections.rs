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
        assert_eq!( a.iter().fold(0, |acc, x| acc + x), 6 );

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
        assert_eq!( v, mv );
        assert_eq!( mv.first().unwrap_or(&0), &1 );
        assert_eq!( mv.last().unwrap_or(&0), &3 );
        assert_eq!( mv.pop().unwrap_or(0), 3 );
        assert_eq!( mv.pop().unwrap_or(0), 2 );
        assert_eq!( mv.pop().unwrap_or(0), 1 );
        assert_eq!( mv.pop(), None );
        assert_eq!( mv.is_empty(), true );
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
        assert_eq!( vd, mvd );
        assert_eq!( mvd.front().unwrap_or(&0), &1 );
        assert_eq!( mvd.back().unwrap_or(&0), &3 );
        assert_eq!( mvd.pop_front().unwrap_or(0), 1 );
        assert_eq!( mvd.pop_back().unwrap_or(0), 3 );
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
        assert_eq!( ll, mll );
        assert_eq!( mll.front().unwrap_or(&0), &1 );
        assert_eq!( mll.back().unwrap_or(&0), &3 );
        assert_eq!( mll.pop_front().unwrap_or(0), 1 );
        assert_eq!( mll.pop_back().unwrap_or(0), 3 );
    }

    #[test]
    fn range() {
        let r = 1..4;
        assert_eq!( r.len(), 3 );
    }

    #[test]
    fn tuple() {
        let t:(u32, u32, u32) = (1, 2, 3);
        assert_eq!( t.0 + t.1 + t.2, 6 );
    }

    #[test]
    fn string() {
        let s = String::from("one");
        assert_eq!( s.len(), 3 );

        let mut ms = String::new();
        ms.push('o');
        ms.push('n');
        ms.push('e');
        assert_eq!( s, ms );
        assert_eq!( ms.pop().unwrap_or('z'), 'e' );
        assert_eq!( ms.pop().unwrap_or('z'), 'n' );
        assert_eq!( ms.pop().unwrap_or('z'), 'o' );
        assert_eq!( ms.pop(), None );

        let mut msx = String::new();
        msx.push_str("two");
        assert_eq!( msx.pop().unwrap_or('z'), 'o' );
        assert_eq!( msx.pop().unwrap_or('z'), 'w' );
        assert_eq!( msx.pop().unwrap_or('z'), 't' );
        assert_eq!( msx.pop(), None );
    }

    #[test]
    fn hashset() {
        use std::collections::HashSet;

        let hs = HashSet::from([1, 2, 3]);
        assert_eq!( hs.contains(&3), true );

        let mut mhs = HashSet::new();
        mhs.insert(1);
        mhs.insert(2);
        mhs.insert(3);
        assert_eq!( hs, mhs );
        assert_eq!( mhs.remove(&3), true );
        assert_eq!( mhs.contains(&3), false );
        assert_eq!( mhs.remove(&2), true );
        assert_eq!( mhs.contains(&2), false );
        assert_eq!( mhs.remove(&1), true );
        assert_eq!( mhs.contains(&1), false );
        assert_eq!( mhs.is_empty(), true );
    }

    #[test]
    fn btreeset() {
        use std::collections::BTreeSet;

        let bts = BTreeSet::from([3, 2, 1]);
        assert_eq!( bts.contains(&3), true );

        let mut mbts = BTreeSet::new();
        mbts.insert(3);
        mbts.insert(2);
        mbts.insert(1);
        assert_eq! ( bts, mbts );
        assert_eq!( mbts.remove(&3), true );
        assert_eq!( mbts.contains(&3), false );
        assert_eq!( mbts.remove(&2), true );
        assert_eq!( mbts.contains(&2), false );
        assert_eq!( mbts.remove(&1), true );
        assert_eq!( mbts.contains(&1), false );
        assert_eq!( mbts.is_empty(), true );
    }

    #[test]
    fn hashmap() {
        use std::collections::HashMap;
        
        let hm = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        assert_eq!( hm.get(&3), Some(&3) );
        assert_eq!( hm.contains_key(&3), true );

        let mut mhm = HashMap::new();
        mhm.insert( 1, 1 );
        mhm.insert( 2, 2 );
        mhm.insert( 3, 3 );
        assert_eq!( hm, mhm );
        assert_eq!( mhm.keys().into_iter().fold(0, |acc, k| acc + k), 6 );
        assert_eq!( mhm.values().into_iter().fold(0, |acc, v| acc + v), 6 );
        assert_eq!( mhm.remove(&1).unwrap_or(0), 1 );
        assert_eq!( mhm.remove(&2).unwrap_or(0), 2 );
        assert_eq!( mhm.remove(&3).unwrap_or(0), 3 );
        assert_eq!( mhm.is_empty(), true );
    }

    #[test]
    fn btreemap() {
        use std::collections::BTreeMap;

        let btm = BTreeMap::from([(1, 1), (2, 2), (3, 3)]);
        assert_eq!( btm.get(&3), Some(&3) );
        assert_eq!( btm.contains_key(&3), true );

        let mut mbtm = BTreeMap::new();
        mbtm.insert(3, 3 );
        mbtm.insert(2, 2 );
        mbtm.insert(1, 1 );
        assert_eq!( btm, mbtm );
        assert_eq!( mbtm.keys().into_iter().fold(0, |acc, k| acc + k), 6 );
        assert_eq!( mbtm.values().into_iter().fold(0, |acc, v| acc + v), 6 );
        assert_eq!( mbtm.remove(&1).unwrap_or(0), 1 );
        assert_eq!( mbtm.remove(&2).unwrap_or(0), 2 );
        assert_eq!( mbtm.remove(&3).unwrap_or(0), 3 );
        assert_eq!( mbtm.is_empty(), true );
    }

    #[test]
    fn binaryheap() {
        use std::collections::BinaryHeap;

        let bh = BinaryHeap::from( [1, 2, 3] );
        assert_eq!( bh.peek().unwrap_or(&0), &3 );

        let mut mbh = BinaryHeap::new();
        mbh.push(1);
        mbh.push(2);
        mbh.push(3);
        assert_eq!( mbh.pop().unwrap_or(0), 3 );
        assert_eq!( mbh.pop().unwrap_or(0), 2 );
        assert_eq!( mbh.pop().unwrap_or(0), 1 );
        assert_eq!( mbh.pop(), None );
        assert_eq!( mbh.is_empty(), true );
    }
}