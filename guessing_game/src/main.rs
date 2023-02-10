use std::io;

fn main() {
    println!("Guess the number!");

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