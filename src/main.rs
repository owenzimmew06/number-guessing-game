// Import necessary libraries (crates and modules)
use std::io; // For handling user input/output
use rand::Rng; // For generating random numbers
use std::cmp::Ordering; // For comparing two values

fn main() {
    // Print a welcome message to the player
    println!("Guess the number!");

    // Generate a secret random number between 1 and 100 (inclusive)
    // `thread_rng()` gives us a random number generator local to the current thread.
    // `gen_range()` generates a number within the specified range.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Start an infinite loop to allow the user to guess multiple times
    loop {
        println!("Please input your guess.");

        // Create a mutable string to store the user's input
        let mut guess = String::new();

        // Read a line from standard input (the keyboard)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handle potential I/O errors

        // Convert the user's input (a String) into a number (u32)
        // We use a `match` expression to handle the two possible outcomes of `.parse()`:
        // 1. `Ok(num)`: Parsing was successful, and `num` is the resulting number.
        // 2. `Err(_)`: Parsing failed (e.g., the user typed "hello").
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue; // Skip the rest of this loop iteration and ask for a new guess
            }
        };

        println!("You guessed: {}", guess);

        // Compare the user's guess to the secret number
        // `cmp` returns an `Ordering` enum, which we can analyze with a `match` expression.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop because the guess was correct
            }
        }
    }
}
