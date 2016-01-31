extern crate rand;

use std::io; //use is a bit like import, io is the library from the standard library
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); //variables are immutable by default but mut makes them mutable.
    //String::new() creates a new, empty String. 

    io::stdin().read_line(&mut guess) // calls stdin which takes user input, then save the input to the guess variable, hence why it needs to be mutable. if immutable, we'd write &guess
        .ok().expect("Failed to read line"); 
        //for error handling. ok() and expect() are methods from the 'io::Result' sub-library. ok() lets us assume the program is 
        //gonna be successful, if error, we just want a generic error not a specific one from Rust in this case.
        // expect(): if err, log the string.
        

    println!("You guessed: {}", guess);
}

