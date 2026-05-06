pub fn delete_and_backspace(s: &mut String) {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '-' {
            result.pop();
            i += 1;
        } else if chars[i] == '+' {
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for operation in v.iter_mut() {
        if operation.contains('+') {
            let parts: Vec<&str> = operation.split('+').collect();

            let left: i32 = parts[0].parse().unwrap();
            let right: i32 = parts[1].parse().unwrap();

            *operation = (left + right).to_string();
        } else if operation.contains('-') {
            let parts: Vec<&str> = operation.split('-').collect();

            let left: i32 = parts[0].parse().unwrap();
            let right: i32 = parts[1].parse().unwrap();

            *operation = (left - right).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_backspace() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();

        delete_and_backspace(&mut a);

        assert_eq!(a, "borrow");
    }

    #[test]
    fn test_do_operations() {
        let mut b = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "5+5".to_owned(),
        ];

        do_operations(&mut b);

        assert_eq!(b, ["4", "5", "7", "10"]);
    }

    #[test]
    fn test_examples() {
        let mut s1 = "helll-o".to_owned();
        let mut s2 = "he+lllo".to_owned();

        delete_and_backspace(&mut s1);
        delete_and_backspace(&mut s2);

        assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
    }
}