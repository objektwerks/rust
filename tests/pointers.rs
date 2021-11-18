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
        let box_sp = Box::new(1);
        assert_eq!( *box_sp, 1 );
    }
}