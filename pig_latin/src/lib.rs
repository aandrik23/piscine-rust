pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(&text.chars().next().unwrap()) {
        return format!("{}ay", text);
    }

    if text.len() >= 3 && &text[1..3] == "qu" {
        return format!("{}{}ay", &text[3..], &text[..3]);
    }

    for (i, c) in text.char_indices() {
        if vowels.contains(&c) {
            return format!("{}{}ay", &text[i..], &text[..i]);
        }
    }
    format!("{}ay", text)
}