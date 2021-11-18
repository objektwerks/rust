#[cfg(test)]
mod pointers {
    #[test]
    fn box_sp() {
        let box_sp = Box::new(3);
        assert_eq!( *box_sp, 3 );
    }
}