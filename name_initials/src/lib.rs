pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let mut result = String::new();

            for word in name.split_whitespace() {
                if let Some(c) = word.chars().next() {
                    result.push(c);
                    result.push('.');
                    result.push(' ');
                }
            }

            result.pop();

            result
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}
}  