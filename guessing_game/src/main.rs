use std::io;
use rand::Rng;

fn main() {
    println!("Dame un numero!");

    let secret_number: i64 = rand::thread_rng().gen_range(1..=100);

    println!("Inserte su numero.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
