pub fn first_subword(s: String) -> String {
    for (i, c) in s.char_indices() {
        if c == '_' || (i != 0 && c.is_uppercase()) {
            return s[..i].to_string();
        }
    }
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
    let s1 = "helloWorld";
    let s2 = "snake_case";
    let s3 = "CamelCase";
    let s4 = "just";

    println!("first_subword({}) = {}", s1, first_subword(s1.to_owned()));
    println!("first_subword({}) = {}", s2, first_subword(s2.to_owned()));
    println!("first_subword({}) = {}", s3, first_subword(s3.to_owned()));
    println!("first_subword({}) = {}", s4, first_subword(s4.to_owned()));
}
}
