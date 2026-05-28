pub fn first_fifty_even_square() -> Vec<i32> {
    let is_even = |n: &i32| n % 2 == 0;
    let square = |n: i32| n * n;

    let result = (1..)
        .filter(is_even)
        .take(50)
        .map(square)
        .collect();

    result
}