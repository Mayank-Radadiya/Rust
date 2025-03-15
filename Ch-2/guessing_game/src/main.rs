// standard input/output library.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng();
    let random_num: u32 = rng.random_range(1..=100);

    loop {
        println!(" \n Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please Enter a num");
                continue;
            }
        };

        match guess.cmp(&random_num) {
            Ordering::Equal => {
                print!("You win");
                break;
            }
            Ordering::Greater => print!("Too high!"),
            Ordering::Less => print!("Too less"),
        }
    }
}
