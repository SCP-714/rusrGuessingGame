use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {

    println!("Please input your guess.");
    
    //adds mutable variable for user input
    let mut guess = String::new();
    
    //error handeling
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
//converts str to num
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {guess}");

    //prints user inputted guess
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
            }
        }
    }
}