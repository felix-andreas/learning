use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    println!("Please input your guess");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed {} and the secret number is {}.", guess, secret_number);
}
