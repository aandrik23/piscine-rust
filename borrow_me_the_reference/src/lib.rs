pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '-' {
            chars.remove(i);

            if i > 0 {
                i -= 1;
                chars.remove(i);
            }
        } else if chars[i] == '+' {
            chars.remove(i);

            if i < chars.len() {
                chars.remove(i);
            }
        } else {
            i += 1;
        }
    }

    *s = chars.into_iter().collect();
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