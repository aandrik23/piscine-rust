use matrix_ops::*;

fn main() {
    let matrix = Wrapper::from([[8, 1], [9, 1]]);
    let matrix_2 = Wrapper::from([[1, 1], [1, 1]]);
    println!("{:?}", matrix + matrix_2);

    let matrix = Wrapper::from([[1, 3], [2, 5]]);
    let matrix_2 = Wrapper::from([[3, 1], [1, 1]]);
    println!("{:?}", matrix - matrix_2);

    let matrix = Wrapper::from([[1, 2], [3, 4]]);
    let matrix_2 = Wrapper::from([[2, 0], [1, 2]]);
    println!("{:?}", matrix * matrix_2);

}