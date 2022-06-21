use std::io;
use std::cmp::Ordering; // trait defines methods to check secret numbers
use rand::Rng; // trait defines methods that random number generators implement


fn main() {
    println!("Guess the number! Choices are 1 - 100"); // Guide user to guess a number

    let secret_number = rand::thread_rng().gen_range(1..101); // getting a range of 1-100 could write as 1..=100 instead

    // println!("The secret number is: {}", secret_number); // printing out the secret number

    loop {
        println!("Please input your guess."); // Guid user to input a number

        let mut guess = String::new(); // Create a mutable String in place of guess. Use ::new(); is UTF-8 encoded text.
    
        io::stdin() // Using the standard input/output library 
            .read_line(&mut guess) // Read line that points to a mutable guess variable. Replacing new String with input value
            .expect("Failed to read line"); // Small error handling
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess); // Printing out the users guess
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), // returns "too small" if the response is a lower number
            Ordering::Greater => println!("Too big"), // returns "too big" if the response is a higher number
            Ordering::Equal => {
                println!("You win!");  // returns "You win" if the response is correct
                break; // Closes the program
            }
        }
    }
    
    
}
