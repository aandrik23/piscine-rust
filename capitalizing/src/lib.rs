pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in input.chars() {
        if ch.is_whitespace() {
            capitalize_next = true;
            result.push(ch);
        } else if capitalize_next {
           for up in ch.to_uppercase() {
                result.push(up);
            }
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_uppercase() {
                ch.to_lowercase().next().unwrap()
            } else {
                ch.to_uppercase().next().unwrap()
            }
        })
        .collect()
}