use case::CaseExt;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let compare_lower = compare.to_lowercase();
    let expected_lower = expected.to_lowercase();

    let is_camel = compare_lower == compare_lower.to_camel();
    let is_snake = compare_lower == compare_lower.to_snake();

    if !is_camel && !is_snake {
        return None;
    }

    let distance = edit_distance::edit_distance(&compare_lower, &expected_lower);

    let percentage = 100 - (distance * 100 / expected_lower.len()).min(100);

    if percentage > 50 {
        Some(format!("{}%", percentage))
    } else {
        None
    }
}