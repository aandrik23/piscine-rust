pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (9.0 / 5.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_to_c() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    }

    #[test]
    fn test_c_to_f() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }
}