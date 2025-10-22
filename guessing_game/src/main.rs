// crates.io is like dependency library
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess. Type quit to give up");

    loop {
        // mutable variable
        // Create a new emty string
        let mut guess = String::new();

        // read_line takes '&mut String'
        // references are immutable by default

        // read_line returns a Result which must be handled by an expect
        // panic! --> expect will crash

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} are placeholders
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Yay you got it!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }

    println!("The number was {}", secret_number);
}
