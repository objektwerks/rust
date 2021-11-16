#[cfg(test)]
mod structs {
    #[test]
    fn structs() {
        struct Person {
            first: String,
            last: String,
            age: u32,
        }
        impl Person {
            fn new(first: String, last: String, age: u32) -> Person {
                Person { first, last, age }
            }
            fn name(&self) -> String {
                return self.first.to_string() + " " + &self.last.to_string();
            }
        }

        let p = Person::new("Fred".to_string(), "Flintstone".to_string(), 27);
        assert_eq!( p.first, "Fred" );
        assert_eq!( p.last, "Flintstone" );
        assert_eq!( p.age, 27 );
        assert_eq!( p.name(), "Fred Flintstone" )
    }

    #[test]
    fn tuple_structs() {
        #[derive(PartialEq, Debug)]
        struct Color(u32, u32, u32);

        const NAVY: Color = Color(0, 0, 128);
        assert_eq!( NAVY, Color(0, 0, 128) )
    }

    #[test]
    fn overloading() {
        use std::ops::Add;

        #[derive(PartialEq, Debug)]
        struct Point {
            x: u32,
            y: u32,
        }
        impl Point {
            fn new(x: u32, y: u32) -> Point {
                Point { x, y }
            }
        }
        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point { x: self.x + other.x, y: self.y + other.y }
            }
        }

        // Add trait impl for Point to + Point structs.
        let sum = Point::new(1, 2) + Point::new(3, 4);
        assert_eq!( sum, Point::new(4, 6) )
    }

    #[test]
    fn destructuring() {
        struct Value {
            v: u32,
        }

        let v = Value { v: 3 };
        let Value { v: dv } = v;
        assert_eq!(3, dv);
    }
}