use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
   //adds mutable variable for user input
    let mut guess = String::new();

    //error handeling
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //prints user inputted guess
    println!("You guessed: {guess}");
}