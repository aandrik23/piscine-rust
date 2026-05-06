pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut skip = 0;

    for ch in s.chars() {
        match ch {
            '+' => skip += 1,
            '-' => { result.pop(); }
            _ => {
                if skip > 0 {
                    skip -= 1;
                } else {
                    result.push(ch);
                }
            }
        }
    }
    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(i) = s.find('+') {
            let left: i32  = s[..i].parse().unwrap();
            let right: i32 = s[i+1..].parse().unwrap();
            *s = (left + right).to_string();
        } else if let Some(i) = s.find('-') {
            let left: i32  = s[..i].parse().unwrap();
            let right: i32 = s[i+1..].parse().unwrap();
            *s = (left - right).to_string();
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