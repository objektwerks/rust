#[cfg(test)]
mod matching {
    #[test]
    fn types() {
        fn square(i: i32) -> i32 {
            match i {
                1 => 1 * 1,
                n => n * n,
            }
        }
        assert_eq!( square(1), 1 );
        assert_eq!( square(4), 16 );
    }

    #[test]
    fn structs() {
        struct Point {
            x: i32,
            y: i32,
        }
        impl Point {
            fn match_points(p: &Point) -> String {
                match p {
                    Point { x: 1, y: _ } => format!("match on x ({}, {})", p.x, p.y),
                    Point { x: _, y: 2 } => format!("match on y ({}, {})", p.x, p.y),
                    Point { x: _, y: _ } => format!("default: ({}, {})", p.x, p.y),
                }
            }
        }
        assert_eq!( Point::match_points( &Point { x: 1, y: 1 } ), "match on x (1, 1)" );
        assert_eq!( Point::match_points( &Point { x: 2, y: 2 } ), "match on y (2, 2)" );
        assert_eq!( Point::match_points( &Point { x: 3, y: 3 } ), "default: (3, 3)" );
    }
}