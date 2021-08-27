use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game.");
    println!("Please choose a number from 1 to 100 ...");

    let secret = rand::thread_rng().gen_range(1..101); //gen range is lower bound .. upper bound + 1

    loop{
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Not input correctly");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number !");
                continue;}
        };

        match number.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!"); 
                break;} 
        }
    }

    print!("The secret number is: {}", secret);
}
