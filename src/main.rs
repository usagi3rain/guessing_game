extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod guess;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        println!("You guessed: {}", guess);
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: guess::Guess = match guess.trim().parse() {
            Ok(num) => guess::Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
