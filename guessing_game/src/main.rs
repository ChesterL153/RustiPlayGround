use std::io;
use rand::Rng;

fn main() {
    println!("Guessing game.");
    println!("Please choose a number");

    let secret = rand::thread_rng().gen_range(1..11); //gen range is lower bound .. upper bound + 1
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Not input correctly");

    println!("You guessed: {} the secret number is : {}", number, secret);
}
