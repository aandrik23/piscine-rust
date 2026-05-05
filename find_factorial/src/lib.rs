pub fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_small_number() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_medium_number() {
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_large_number() {
        assert_eq!(factorial(19), 121645100408832000);
    }
}