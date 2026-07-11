use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is : {secret_number}");

    let mut tryy = 0;

    loop {
        tryy += 1;
        println!("Please input your guess (Try : {tryy}) : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal");
                println!("Congratulations you took {tryy} guesses!");
                break;
            }
        }
    }
}
