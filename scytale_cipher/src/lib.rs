pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 {
        return String::new();
    }

    let mut chars: Vec<char> = message.chars().collect();

    while chars.len() % i != 0 {
        chars.push(' ');
    }

    let rows = chars.len() / i;

    let mut result = String::new();

    for col in 0..i {
        for row in 0..rows {
            let index = row * i + col;
            result.push(chars[index]);
        }
    }

    result.trim_end().to_string()
}