// Import necessary modules from the standard library
// io module for input/output operations
// Write trait for flush() functionality
use std::io::{self, Write};
// Import Ordering enum for comparing values
use std::cmp::Ordering;
// Import Rng trait for random number generation
use rand::Rng;
// Import clap for command line argument parsing
use clap::Parser;

/// A simple number guessing game
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The maximum number to guess (minimum is always 1)
    #[arg(short = 'n', long, default_value_t = 100)]
    pub max: u32,

    /// Maximum number of attempts allowed
    #[arg(short = 'a', long, default_value_t = 10)]
    pub max_attempts: u32,
}

fn main() {
    let args = Args::parse();

    // Display welcome message and game instructions
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and {}.", args.max);
    println!("You have {} attempts to guess the number.", args.max_attempts);

    // Generate a random number between 1 and the maximum number
    let secret_number = rand::thread_rng().gen_range(1..=args.max);
    // Initialize counter for tracking number of attempts
    let mut attempts = 0;

    // Main game loop
    loop {
        if attempts >= args.max_attempts {
            println!("Game Over! You've used all your attempts.");
            println!("The number was: {}", secret_number);
            break;
        }

        // Prompt user for input
        println!("Please input your guess (attempt {}/{}): ", attempts + 1, args.max_attempts);
        io::stdout().flush().unwrap();

        // Create a new empty string to store user input
        let mut guess = String::new();

        // Read user input from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input string to an unsigned 32-bit integer
        // trim() removes whitespace and newlines
        // parse() converts the string to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > args.max {
                    println!("Please enter a number between 1 and {}!", args.max);
                    continue;
                }
                num
            },
            Err(_) => {      // If parsing fails (invalid input)
                println!("Please enter a valid number!");
                continue;    // Skip to next iteration of the loop
            }
        };

        // Increment the attempts counter
        attempts += 1;

        // Compare the user's guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),      // Guess is lower than secret number
            Ordering::Greater => println!("Too big!"),     // Guess is higher than secret number
            Ordering::Equal => {                           // Guess matches secret number
                println!("Congratulations! You guessed the number in {} attempts!", attempts);
                break;                                     // Exit the game loop
            }
        }
    }
}

// Declare the tests module
#[cfg(test)]
mod tests; 