pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut s1: Vec<char> = s1.to_lowercase().chars().collect();
    let mut s2: Vec<char> = s2.to_lowercase().chars().collect();
    s1.sort();
    s2.sort();
    s1 == s2
}