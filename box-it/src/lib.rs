pub fn parse_into_boxed(s:String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|num| {
            if num.ends_with('k') {
                let without_k = num.trim_end_matches('k');
                let value = without_k.parse::<f32>().unwrap();
                Box::new((value * 1000.0) as u32)
            } else {
                let value = num.parse::<u32>().unwrap();
                Box::new(value)
            }
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter()
        .map(|boxed_num| *boxed_num)
        .collect()
}