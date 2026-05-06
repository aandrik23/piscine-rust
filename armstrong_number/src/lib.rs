pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let digits = nb.to_string().len() as u32;

    let mut sum = 0;
    let mut n = nb;

    while n > 0 {
        let digit = n % 10;

        sum += digit.pow(digits);

        n /= 10;
    }

    if sum == nb {
        Some(nb)
    } else if nb == 0 {
        Some(0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armstrong_numbers() {
        assert_eq!(is_armstrong_number(0), Some(0));
        assert_eq!(is_armstrong_number(1), Some(1));
        assert_eq!(is_armstrong_number(153), Some(153));
        assert_eq!(is_armstrong_number(370), Some(370));
        assert_eq!(is_armstrong_number(371), Some(371));
        assert_eq!(is_armstrong_number(407), Some(407));
    }

    #[test]
    fn test_not_armstrong_numbers() {
        assert_eq!(is_armstrong_number(400), None);
        assert_eq!(is_armstrong_number(198), None);
    }
}
