#[cfg(test)]
mod strings {
    #[test]
    fn string() {
        let mut ff = String::new();
        ff.push_str("fred ");
        ff.push_str("flintstone");

        assert_eq!( ff, "fred flintstone" );

        let w = "wilma".to_string();
        let f = "flintstone".to_string();
        let wf = format!("{} {}", w, f);

        assert_eq!( wf, "wilma flintstone" );

        let is = 3.to_string();

        assert_eq!( is, "3" );
    }

    #[test]
    fn str() {
        let model = "abc".to_owned() + "123";

        assert_eq!( model, "abc123" );
    }

    #[test]
    fn slice() {
        let br = String::from("barney rebel");
        let barney = &br[0..6];
        let rebel = &br[7..12];

        assert_eq!( barney, "barney" );
        assert_eq!( rebel, "rebel" );
    }
}