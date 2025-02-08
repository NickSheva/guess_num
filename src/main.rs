use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your number is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less"),
            Ordering::Greater => println!("Too bigger"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

