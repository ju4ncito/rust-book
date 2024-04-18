use rand::Rng; // Rng trait defines methods that rngs implement (must be in scope)
use std::io; // input/output library into the file's scope, it comes from the standart library (std)

use std::cmp::Ordering;
fn main() {
    //main function
    println!("GuessTheNumber!"); //macro that prints to the screen

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Gen number: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new(); //mut is a variable where we are saving a String

        // In Rust, variables are IMMUTABLE by default, so we have to explicitly
        // indicate inside the prgram that the variable is available to be changed

        io::stdin()
            .read_line(&mut guess) //calls read_line method and where to put the string
            .expect("Failed to read line"); //readline returns result (enum) of Ok and Err. Err shows expect, Ok shows Ok answet bytes

        // std::io::stdin (how to use std without importing to the whole scope. :: works like . in js)

        // fix string AND shadowing

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try with a number");
                continue;
            }
        }; // with expect we are covering
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
