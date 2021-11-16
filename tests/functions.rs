#[cfg(test)]
mod functions {
    #[test]
    fn closure() {
        let a = [1, 2, 3];
        assert_eq!( a.iter().map(|x| x * x).last().unwrap_or(0), 9 );

        type Multiplier = u32;
        let y: Multiplier = 3;
        assert_eq!( a.iter().map(|x| x * y).last().unwrap_or(0), 9 );
    }

    #[test]
    fn higher_order() {
        fn square(i: u32) -> u32 {
            return i * i;
        }

        let a = [1, 2, 3];
        assert_eq!( a.iter().map(|x| square(*x)).last().unwrap_or(0), 9 );
    }

    #[test]
    fn fn_pointer() {
        fn sum(x: u32, y: u32) -> u32 {
            x + y
        }

        fn product(x: u32, y: u32) -> u32 {
            x * y
        }

        fn combine(f: fn(u32, u32) -> u32,
                   g: fn(u32, u32) -> u32,
                   x: u32,
                   y: u32) -> u32 {
            f(x, y) + g(x, y)
        }

        assert_eq!( combine(sum, product, 3, 3), 15 );
    }
}