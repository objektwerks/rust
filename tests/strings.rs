#[cfg(test)]
mod strings {
    #[test]
    fn string() {
        let mut s = String::new();
        s.push_str("fred ");
        s.push_str("flintstone");
        assert_eq!(s, "fred flintstone");

        let b = "barney ".to_string();
        let r = "rebel".to_string();
        let br = b + &r;
        assert_eq!(br, "barney rebel");

        let i = 3;
        let is = i.to_string();
        assert_eq!(is, "3");

        let w = "wilma".to_string();
        let f = "flintstone".to_string();
        let wf = format!("{} {}", w, f);
        assert_eq!(wf, "wilma flintstone");

        let model: String = String::from("abc");
        let number: String = String::from("123");
        let id: String = model + &number;
        assert_eq!(id, "abc123");
    }
}