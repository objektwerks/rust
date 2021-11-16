#[cfg(test)]
mod results {
    use std::io::Error;
    use std::io::Read;
    use std::fs::File;

    #[test]
    fn ok() {
        let result = File::open("Cargo.toml");
        assert!(result.is_ok());
    }

    #[test]
    fn err() {
        let result = File::open("Crate.toml");
        assert!(result.is_err());
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
            Err(err) => assert!(!err.to_string().is_empty()),
        }
        match read_file("Cargo.toml") {
            Ok(contents) => assert!(!contents.is_empty()),
            Err(err) => panic!("Failed to open Cargo.toml: {}", err),
        }
    }
}