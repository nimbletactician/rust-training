use std::io;
// prelude: println, String
fn main() {
    println!("Guess the number!");
    println!("Please input the guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().unwrap();
    println!("You guess: {guess}")
}
