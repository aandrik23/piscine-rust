pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_ascii_digit())
            .unwrap()

    });

    words.iter().map(|word| {
        word.chars().filter(|c| !c.is_ascii_digit()).collect::<String>()
    })
    .collect::<Vec<String>>()
    .join(" ")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn main() {
    println!("{}", arrange_phrase("is2 Thi1s T4est 3a"));
}
}