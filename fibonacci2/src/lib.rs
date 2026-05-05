pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn main() {
    println!(
        "The element in the position {} in fibonacci series is {}",
        2,
        fibonacci(2)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        4,
        fibonacci(4)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        22,
        fibonacci(22)
    );
    println!(
        "The element in the position {} in fibonacci series is {}",
        20,
        fibonacci(20)
    );
}
}