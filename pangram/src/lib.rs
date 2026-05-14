pub fn is_pangram(s: &str) -> bool {
    let lowercase = s.to_lowercase();
    ('a'..='z').all(|c| lowercase.contains(c))
}