use rand::{rngs::ThreadRng, Rng};
use std::io::{stdin, stdout, Write};

fn main() {
    let mut rng = rand::thread_rng();
    greet();

    let random_number = generate_number(&mut rng);

    println!("{}", random_number);
    get_guess(random_number);
}

fn greet() {
    println!("Welcome to the number guessing game!\n");
    println!("A random number will be chosen and you will");
    print!("try to guess! Press ENTER to start...");
    stdout().flush().unwrap();

    read_input();
    println!("\n");
}

fn generate_number(rng: &mut ThreadRng) -> u8 {
    let range: std::ops::RangeInclusive<u8> = 1..=20;
    rng.gen_range(range)
}

fn get_guess(number: u8) {
    print!("Guess the number: ");
    stdout().flush().unwrap();

    match check_guess(parse_u8(), number) {
        Some(_) => println!("You won!"),
        None => println!("You lost :("),
    }
}

fn check_guess(guess: u8, number: u8) -> Option<u8> {
    if guess == number {
        return Some(number);
    }
    None
}

fn read_input() -> String {
    let mut buffer = String::new();
    loop {
        match stdin().read_line(&mut buffer) {
            Ok(_) => return buffer.trim().to_owned(),
            Err(_) => {
                print!("Invalid input. Try again: ");
                stdout().flush().unwrap();
                continue;
            }
        }
    }
}

fn parse_u8() -> u8 {
    loop {
        let input = read_input();
        match input.parse::<u8>() {
            Ok(num) => break num,
            Err(_) => {
                print!("Invalid input. Try again: ");
                stdout().flush().unwrap();
                continue;
            }
        }
    }
}
