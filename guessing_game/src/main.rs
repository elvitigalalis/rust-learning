use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        
        println!("----------------");
        print!("Please input your guess: ");
        let _ = io::stdout().flush();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed  to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        println!("You guessed: {}", guess);

        let closeness = f64::from((guess - secret_number).abs()) / 100.0 * 255.0;
        let closeness = closeness as u8;

        // println!("Closeness: {}", closeness); 

        // Customized the color of the "Too small!" and "Too big!" messages to align with how close the guess is to the secret number. Gray for far away, red for close.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!\n".truecolor(255 - closeness, 0, 0)),
            Ordering::Greater => println!("{}", "Too big!\n".truecolor(255 - closeness, 0, 0)),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}