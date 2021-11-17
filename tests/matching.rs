#[cfg(test)]
mod matching {
    #[test]
    fn types() {
        fn square(i: u32) -> u32 {
            match i {
                1 => 1 * 1,
                n => n * n,
            }
        }

        assert_eq!( square(1), 1 );
        assert_eq!( square(4), 16 );
    }

    #[test]
    fn guards() {
        fn is_even(i: u32) -> bool {
            match i {
                i if i % 2 == 0 => true,
                _ => false,
            }
        }

        assert_eq!( is_even(2), true );
        assert_eq!( is_even(3), false );
    }

    #[test]
    fn structs() {
        struct Point {
            x: u32,
            y: u32,
        }
        impl Point {
            fn new(x: u32, y: u32) -> Point {
                Point { x, y }
            }
            fn match_on_point(self) -> String {
                match self {
                    Point { x: 1, y: _ } => format!("match on x ({}, {})", self.x, self.y),
                    Point { x: _, y: 2 } => format!("match on y ({}, {})", self.x, self.y),
                    Point { x: _, y: _ } => format!("default: ({}, {})", self.x, self.y),
                }
            }
        }

        let px = Point::new(1, 1);
        assert_eq!(px.match_on_point(), "match on x (1, 1)" );

        let py = Point::new(2, 2);
        assert_eq!(py.match_on_point(), "match on y (2, 2)" );

        let pxy = Point::new(3, 3);
        assert_eq!(pxy.match_on_point(), "default: (3, 3)" );
    }

    #[test]
    fn binding() {
        enum Ping {
            Id { id: u32 },
        }

        fn match_on_ping(ping: Ping) -> u32 {
            match ping {
                Ping::Id {
                    id: id_var @ 1..=3,
                } => id_var * 2,
                Ping::Id {
                    id: id_var @ 4..=6,
                } => id_var * 3,
                Ping::Id {
                    id
                } => id,
            }
        }

        assert_eq!( match_on_ping( Ping::Id { id: 3 } ), 6 );
        assert_eq!( match_on_ping( Ping::Id { id: 6 } ), 18 );
        assert_eq!( match_on_ping( Ping::Id { id: 9 } ), 9 );
    }
}