pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_question = text.ends_with('?');
    let has_letters = text.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

    
    if is_yelling && is_question {
        "Quiet, I am thinking!"
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else if is_question {
        "Sure."
    } else {
        "Interesting"
    }
}