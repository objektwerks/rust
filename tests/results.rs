#[cfg(test)]
mod results {
    use std::io::Read;

    #[test]
    fn result() {
        use std::fs::File;

        let err = File::open("crate.toml");
        assert!(err.is_err());

        let ok = File::open("Cargo.toml");
        assert!(ok.is_ok());

        let mut file = ok.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert!(!contents.is_empty());

        use std::io::Error;
        fn read_file(file: &str) -> Result<String, Error> {
            let mut f = File::open(file)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        assert!(read_file("Cargo.toml").ok().is_some())
    }
}