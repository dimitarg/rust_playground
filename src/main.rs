use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Number guessing game.");

    let mut rng = rand::thread_rng();
    // this explicit type annotation would have been inferred by virtue of the `cmp` usage below
    let our_number: u32 = rng.gen_range(1..=100);

    loop {
        // equivalent to c-style while(true) I assume.
        println!("Please enter your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Line of input expected.");

        // apparently shadowing is considered idiomatic, especially when doing datatype conversions?
        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                // this compiles because `continue` has type `!` (read - Never) (similar to haskell `Void` / Scala `Nothing`)
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&our_number) {
            Ordering::Less => print!("Number too small."),
            Ordering::Equal => {
                println!("You guessed correctly. You win!.");
                break; // breaks the enclosing `loop`
            }
            Ordering::Greater => println!("Number too big."),
        }
    }
}
