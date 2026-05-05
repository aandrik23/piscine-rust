use std::io;
fn main() {
    let riddle = "I am the begging of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "the letter e";

    let mut attempts = 0;

    loop {
        println!("{}", riddle);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        attempts += 1;
        
        let user_input = input.trim().to_lowercase();

        if user_input == answer {
            println!("Correct! You solved the riddle in {} attempts.", attempts);
            break;
        }
    }
}