fn main() {
    let rust = "Rust!";
    println!("Hello, {}", rust);
}

#[cfg(test)]
mod tests {
    #[test]
    fn array() {
        let mut a = [1, 2, 3];
        a.reverse();
        assert_eq!( a, [3, 2, 1] );
    }

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];
        assert_eq!( v[2], 3 );
    }

    #[test]
    fn tuple() {
        let t = (1, 2, 3);
        assert_eq!( t.2, 3 );
    }
}