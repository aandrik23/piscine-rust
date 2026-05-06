pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (
        c,
        (c as f64).exp(),
        (c.abs() as f64).ln(),
    )
}

pub fn str_function(a: String) -> (String, String) {
    let result = a
        .split_whitespace()
        .map(|n| {
            let num: f64 = n.parse().unwrap();
            num.exp().to_string()
        })
        .collect::<Vec<String>>()
        .join(" ");

    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let logs = b
        .iter()
        .map(|n| ((*n).abs() as f64).ln())
        .collect();

    (b, logs)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn main() {
    let a = "1 2 4 5 6".to_owned();
    let b = vec![1, 2, 4, 5];
    let c = 0;

    println!("{:?}", nbr_function(c));
    println!("{:?}", vec_function(b));
    println!("{:?}", str_function(a));
}
}