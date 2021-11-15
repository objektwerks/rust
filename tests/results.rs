#[cfg(test)]
mod results {
    use std::io::Error;
    use std::io::Read;
    use std::fs::File;

    #[test]
    fn ok() {
        let ok = File::open("Cargo.toml");
        assert!(ok.is_ok());
    }

    #[test]
    fn err() {
        let err = File::open("Crate.toml");
        assert!(err.is_err());
    }

    #[test]
    fn file() {
        let mut file = File::open("Cargo.toml").unwrap();
        let mut contents = String::new();
        let usize = file.read_to_string(&mut contents).unwrap();
        assert_ne!(usize, 0);
        assert!(!contents.is_empty());
    }

    #[test]
    fn result() {
        fn read_file(path: &str) -> Result<String, Error> {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }

        assert!(read_file("Crate.toml").is_err());
        assert!(read_file("Cargo.toml").is_ok());

        match read_file("Crate.toml") {
            Ok(contents) => panic!("Opened nonexistent Crate.toml: {}", contents),
            Err(failure) => assert!(!failure.to_string().is_empty()),
        }
        match read_file("Cargo.toml") {
            Ok(contents) => assert!(!contents.is_empty()),
            Err(failure) => panic!("Failed to open Cargo.toml: {}", failure),
        }
    }
}