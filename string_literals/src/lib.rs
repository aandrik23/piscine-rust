pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert_eq!(is_empty(""), true);
    }

    #[test]
    fn test_is_ascii() {
        assert_eq!(is_ascii("rust"), true);
    }

    #[test]
    fn test_contains() {
        assert_eq!(contains("rust", "ru"), true);
    }

    #[test]
    fn test_split_at() {
        assert_eq!(split_at("rust", 2), ("ru", "st"));
    }

    #[test]
    fn test_find() {
        assert_eq!(find("rust", 'u'), 1);
    }
}