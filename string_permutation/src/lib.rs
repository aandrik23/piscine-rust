
use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.chars().count() != s2.chars().count() {
        return false;
    }

    let mut counts = HashMap::new();

    for ch in s1.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    for ch in s2.chars() {
        match counts.get_mut(&ch) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    counts.remove(&ch);
                }
            }
            None => return false,
        }
    }

    counts.is_empty()
}