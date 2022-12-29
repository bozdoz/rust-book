use std::{ io, cmp::Ordering };

use rand::Rng;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("please input a guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faiiled to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("gt"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}