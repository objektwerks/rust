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
            fn match_on_point(&self) -> String {
                match self {
                    Point { x: 1, y: _ } => format!("match on x ({}, {})", self.x, self.y),
                    Point { x: _, y: 2 } => format!("match on y ({}, {})", self.x, self.y),
                    Point { x: _, y: _ } => format!("default: ({}, {})", self.x, self.y),
                }
            }
        }
        let px = &Point { x: 1, y: 1 };
        assert_eq!(px.match_on_point(), "match on x (1, 1)" );

        let py = &Point { x: 2, y: 2 };
        assert_eq!(py.match_on_point(), "match on y (2, 2)" );

        let pxy = &Point { x: 3, y: 3 };
        assert_eq!(pxy.match_on_point(), "default: (3, 3)" );
    }
}