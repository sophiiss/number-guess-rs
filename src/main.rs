use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use rand::{rngs::ThreadRng, Rng};
use std::{
    io::{stdin, stdout, Write},
    thread,
    time::Duration,
};

fn main() {
    thread::sleep(Duration::from_secs(1));
    clear_screen();

    let mut rng = rand::thread_rng();
    let max_attempts = 5;
    greet();

    let random_number = generate_number(&mut rng);

    get_guess(&random_number, &max_attempts);
}

fn clear_screen() {
    let (_, rows) = size().unwrap();
    enable_raw_mode().unwrap();
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, rows - 1)).unwrap();
    disable_raw_mode().unwrap();
}

fn greet() {
    println!("Welcome to the number guessing game!\n");
    println!("A random number will be chosen and you will");
    print!("try to guess! Press ENTER to start...");
    stdout().flush().unwrap();

    read_input();
    clear_screen();
    println!("You've got 5 guesses!");
}

fn generate_number(rng: &mut ThreadRng) -> u8 {
    let range: std::ops::RangeInclusive<u8> = 1..=20;
    rng.gen_range(range)
}

fn get_guess(number: &u8, max_attempts: &i32) {
    let mut current_attempts = 1;

    loop {
        print!(
            "({}/{}) Guess the number: ",
            &current_attempts, max_attempts
        );
        stdout().flush().unwrap();

        let guess = parse_u8();

        if guess == *number {
            break game_won(number);
        } else {
            if current_attempts >= 5 {
                break game_lost(number);
            }
            current_attempts += 1;
            clear_screen();
            println!("\nWrong answer, but here's a tip:");
            println!("{}\n", send_tip(number, guess));
            continue;
        }
    }
}

fn send_tip(number: &u8, last_guess: u8) -> String {
    if last_guess > *number {
        format!("The number is less than {}", last_guess)
    } else {
        format!("The number is greater than {}", last_guess)
    }
}

fn game_won(number: &u8) {
    clear_screen();
    println!("\nYou won!\nThe number was {}!", *number);
}

fn game_lost(number: &u8) {
    clear_screen();
    println!("\nYou lost :(\nThe number was {}.", *number);
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
