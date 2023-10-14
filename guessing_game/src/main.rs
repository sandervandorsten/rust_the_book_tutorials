use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
use std::process::exit;

fn main() {
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        // Read the users input.
        println!("Please input your guess:");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");

        // Try to parse the input to an int.
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        // Check if the guessed number is correct. If so; exit.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Congratz!!");
                exit(0);
            }
        };
    }
}
