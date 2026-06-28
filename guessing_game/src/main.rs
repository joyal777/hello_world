use std::io;

use rand::Rng;
fn main() {
    println!("Guess the number!");

    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    println!("You guessed: {guess}");

    println!("The secret number is: {secret_number}");

    if guess == secret_number.to_string() {
        println!("You win!");
    } else {
        println!("You lose!");
    }
    
}

    