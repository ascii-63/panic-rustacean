use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess: ");
        let mut guess = String::new(); // mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
        .trim() // Remove whitespace
        .parse() // Convert
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                print!("You win!");
                break;
            }
        }
    }
}
