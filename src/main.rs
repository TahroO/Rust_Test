use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_counter = 0;

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_counter = guess_counter + 1;
            }
            Ordering::Greater => {
                println!("To high!");
                guess_counter = guess_counter +1;
            },
            Ordering::Equal => {
                println!("You win!");
                println!("You needed: {guess_counter} tries");
                break;
            },
        }
    }
}