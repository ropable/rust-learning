// Use the external crate rand. This is also equiv to a ``use rand``
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Get a copy of the random number generator.
    // Because we put use rand::Rng above, it has a gen_range() method available.
    // This takes 2 numbers and generates a random number between them,
    // lower inclusive and upper exclusive.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("Secret number: {}", secret_number);

    println!("Guess the number between 1 and 100!");

    loop {
        println!("Please input your guess");

        // Define a mutable variable to hold the guess.
        let mut guess = String::new();

        // io::stdin().read_line() takes a &mut String as an argument,
        // and returns an io::Result that we should use.
        // We use .expect() to crash and return a message on error.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input to a number. We can re-use the guess variable name.
        // Here, we specify that the new variable is an unsigned 32-bit number.
        // The .trim() method on the string removes any whitespace/newlines.
        // The .parse() method parses a string into another type, here a number.
        // parse returns a Result, an enum.
        // We use a match statement
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);

        // The cmp() method can be used on anything that can be compared,
        // and it takes a reference to the thing you want to compare it to.
        // It returns the Ordering type we imported earlier.
        match guess.cmp(&secret_number) {
            // Ordering is an enum with 3 variants: Less, Greater and Equal.
            Ordering::Less      => println!("Too low!"),
            Ordering::Greater   => println!("Too high!"),
            Ordering::Equal     => {
                println!("Correct! You win!");
                // break will exit the loop.
                break;
            }
        }
    }
}
