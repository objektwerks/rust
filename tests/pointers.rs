#[cfg(test)]
mod pointers {
    #[test]
    fn deref() {
        let x = 1;
        let y = &x;

        assert_eq!( x, 1 );
        assert_eq!( *y, 1 );
    }

    #[test]
    fn box_sp() {
        let x = 1;
        let y = Box::new(x);

        assert_eq!( x, 1 );
        assert_eq!( *y, 1 );
    }
}