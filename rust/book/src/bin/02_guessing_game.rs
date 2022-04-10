use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please guess a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Err(_) => {
                println!("could not parse '{}'. Try again:", guess.trim());
                continue;
            }
            Ok(x) => x,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => break,
            Ordering::Less => println!("Your number is too small! Try again:"),
            Ordering::Greater => println!("Your number is too big! Try again:"),
        };
    }

    println!("You win!")
}
