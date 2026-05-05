pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x/y, x%y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(10, 3);
        assert_eq!(result, (3, 1));
    }
}
