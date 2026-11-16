use std::io;
use rand::prelude::*;
use std::cmp::Ordering;
fn main(){

    let secret_number  = rand::rng().random_range(1..=100);
    // println!("The secret number is {secret_number}");
    loop {
        println!("Enter a number to guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to guess");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly");
                break;
            }
        }

    }
}
