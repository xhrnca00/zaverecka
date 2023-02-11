use std::{cmp::Ordering, error::Error, io::Write, ops::RangeInclusive};

use rand::prelude::*;

const MIN: i32 = 1;
const MAX: i32 = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut guess_count = 0;
    //* Generate a random number from 1 up to 1000.
    let number = rand::thread_rng().gen_range(MIN..=MAX);
    // Print the game instructions
    println!("I am thinking a number from {MIN} to {MAX}.");
    println!("Your goal is to guess it. I will tell you if your guess is too high or too low.");
    loop {
        println!("{}", "-".repeat(20));
        let mut guess = String::new();
        //* getting a guess
        print!("Enter your guess: ");
        std::io::stdout().flush()?; // flush the buffer so only the input is read
        std::io::stdin().read_line(&mut guess)?;
        //* error handling + conversion
        // shadowing the guess variable to i32
        let guess = match guess.trim().parse() {
            Ok(num) if (MIN..=MAX).contains(&num) => num,
            _ => {
                println!("Please enter a whole number between {MIN} and {MAX}.");
                continue;
            }
        };
        guess_count += 1;
        //* comparing the guess to the number
        match guess.cmp(&number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You guessed it! The number was {number}.");
                println!("It took you {guess_count} guesses.");
                break;
            }
        }
    }
    Ok(())
}
