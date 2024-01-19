use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("DEBUG: The secret Number is: {secret_number}");

    println!("Welcome to Murasko's Guessing Game!");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low! ({guess})"),
            Ordering::Greater => println!("You guessed too high! ({guess})"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
