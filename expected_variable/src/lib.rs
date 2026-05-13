use case::CaseExt;

fn edit_distance(a: &str, b: &str) -> usize {
    let mut differences = 0;

    let min_len = a.len().min(b.len());

    for (c1, c2) in a.chars().take(min_len).zip(b.chars()) {
        if c1 != c2 {
            differences += 1;
        }
    }

    differences + a.len().max(b.len()) - min_len
}

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    if !compare.is_camel() && !compare.is_snake() {
        return None;
    }

    let compare = compare.to_lowercase();
    let expected = expected.to_lowercase();

    let distance = edit_distance(&compare, &expected);

    let max_len = expected.len().max(compare.len());

    let similarity = ((max_len - distance) * 100) / max_len;

    if similarity > 50 {
        Some(format!("{}%", similarity))
    } else {
        None
    }
}