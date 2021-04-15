fn main() {
    let rust = "Rust!";
    println!("Hello, {}", rust);
}

#[cfg(test)]
mod tests {
    #[test]
    fn array() {
        let mut v = [1, 2, 3];
        v.reverse();
        assert_eq!( v, [3, 2, 1] );
    }
}