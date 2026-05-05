pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str{
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut groceries = vec![
            "milk".to_string(),
            "bread".to_string(),
        ];

        insert(&mut groceries, "cheese".to_string());

        assert_eq!(
            groceries,
            vec![
                "milk".to_string(),
                "bread".to_string(),
                "cheese".to_string(),
            ]
        );
    }

    #[test]
    fn test_at_index() {
        let groceries = vec![
            "fruits".to_string(),
            "juice".to_string(),
            "bread".to_string(),
        ];

        let item = at_index(&groceries, 1);

        assert_eq!(item, "juice");
    }
}