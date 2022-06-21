use std::io;

fn main() {
    println!("Guess the number!"); // Guide user to guess a number

    println!("Please input your guess."); // Guid user to input a number

    let mut guess = String::new(); // Create a mutable String in place of guess. Use ::new(); is UTF-8 encoded text.

    io::stdin() // Using the standard input/output library 
        .read_line(&mut guess) // Read line that points to a mutable guess variable. Replacing new String with input value
        .expect("Failed to read line"); // Small error handling

    println!("You guessed: {}", guess); // Printing out the users guess
}
