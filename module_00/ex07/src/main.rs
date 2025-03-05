fn strpcmp(query: &[u8], pattern: &[u8]) -> bool{
 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strpcmp() {
        assert!(strpcmp(b"abc", b"abc"));
        assert!(strpcmp(b"abcd", b"ab*"));
        assert!(!strpcmp(b"cab", b"ab*"));
        assert!(strpcmp(b"dcab", b"*ab"));
        assert!(!strpcmp(b"abc", b"*ab"));
        assert!(strpcmp(b"ab000cd", b"ab*cd"));
        assert!(strpcmp(b"abcd", b"ab*cd"));
        assert!(strpcmp(b"", b"****"));
    }
}

fn main() {
    println!("Hello, world!");
}
