use std::io; // include standard io library
use std::cmp::Ordering; // include standard cmp library
use rand::Rng; // library crate downloaded from the registry (crates.io)

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Make a guess!");
    println!("Please input your guess.");
    
    loop { // loop        
        // note that a variable is considered to be immutable by default in rust
        // let indicates local variable, and mut indicates that the value is mutable
        let mut guess = String::new(); 
        
        // everything is considered immutable by default, references are not an exception
        io::stdin()
            .read_line(&mut guess) // reference operator & followed by mut keyword
            .expect("Failed to read line"); // error handling
        let guess: u32 = match guess.trim().parse() { // parse string into integer
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) { // match expression matches the result of the given expression to arms in the body
            Ordering::Less => println!("Too small!"), // first arm
            Ordering::Greater => println!("Too big!"), // second
            Ordering::Equal => { // last arm
                println!("Correct!");
                break;
            },
        }
    }

}
