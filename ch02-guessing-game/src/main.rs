use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new(); // mutable variable

        io::stdin()
            .read_line(&mut guess) // read input from the user
            .expect("Failed to read line"); // error handling

        println!("Your guess was: {}", guess.trim());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("Please input your guess.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit the loop
            }
        }
    }
}
