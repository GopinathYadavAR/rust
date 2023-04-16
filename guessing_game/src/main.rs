// add required packages
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Guess Game !!!");
    println!("Enter the number");
    let mut guess_number = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    io::stdin()
        .read_line(&mut guess_number)
        .expect("Failed to read the input");
    let guess_number: u32 = guess_number.trim()
        .parse().expect("Please type a number");

    match guess_number.cmp(&secret_number) {
        Ordering::Less => println!("less than secret number"),
        Ordering::Equal => println!("guess number and secret number are same"),
        Ordering::Greater => println!("Guess number is greater than secret number"),
    };
    println!("Guessed number = {guess_number}");
    println!("secret number = {secret_number}");
}
