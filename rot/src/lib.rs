pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| {
        if c.is_ascii_lowercase() {
            rotate_char(c, key, b'a')
        } else if c.is_ascii_uppercase() {
            rotate_char(c, key, b'A')
        } else {
            c
        }
    })
    .collect()
}

fn rotate_char(c: char, key: i8, base: u8) -> char {
    let alpha_index = c as i8 - base as i8;

    let rotated = (alpha_index + key).rem_euclid(26);

    (base + rotated as u8) as char
}