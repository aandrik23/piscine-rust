pub fn to_url(s: &str) -> String {
    s.replace(" ","%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_url() {
        let s = "Hello World";

        assert_eq!(to_url(s), "Hello%20World");
    }
}