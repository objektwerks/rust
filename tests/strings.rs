#[cfg(test)]
mod strings {
    #[test]
    fn string() {
        let mut fred_flintstone = String::new();
        fred_flintstone.push_str("fred ");
        fred_flintstone.push_str("flintstone");
        assert_eq!(fred_flintstone, "fred flintstone" );

        let wilma = "wilma".to_string();
        let flintstone = "flintstone".to_string();
        let wilima_flintstone = format!("{} {}", wilma, flintstone);
        assert_eq!(wilima_flintstone, "wilma flintstone" );

        let betty = String::from("betty ");
        let rubble = String::from("rubble");
        let betty_rubble = betty + &*rubble;
        assert_eq!(betty_rubble, "betty rubble" );

        let is_3 = 3.to_string();
        assert_eq!( is_3, "3" );
    }

    #[test]
    fn str() {
        let model = "abc".to_owned() + "123";
        assert_eq!( model, "abc123" );
    }

    #[test]
    fn slice() {
        let barney_rubble = String::from("barney rubble");
        let barney = &barney_rubble[0..6];
        let rubble = &barney_rubble[7..barney_rubble.len()];
        assert_eq!( barney, "barney" );
        assert_eq!( rubble, "rubble" );

        let barney_rubble_x = &barney_rubble[0..barney_rubble.len()];
        let barney_rebel_xx = &barney_rubble[..];
        assert_eq!( barney_rubble_x, "barney rubble" );
        assert_eq!( barney_rebel_xx, "barney rubble" );

        let a = [1, 2, 3];
        let slice = &a[1..a.len()];
        assert_eq!( slice, &[2, 3] );
    }
}