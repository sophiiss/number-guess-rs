use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use rand::{rngs::ThreadRng, Rng};
use std::{
    fmt,
    io::{stdin, stdout, Write},
    ops::RangeInclusive,
    thread,
    time::Duration,
};

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Difficulty::Easy => write!(f, "'easy'"),
            Difficulty::Medium => write!(f, "'medium'"),
            Difficulty::Hard => write!(f, "'hard'"),
        }
    }
}

fn main() {
    thread::sleep(Duration::from_secs(1));
    clear_screen();

    greet();

    start_game();
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
}

fn start_game() {
    loop {
        let difficulty = select_difficulty();
        clear_screen();

        let mut rng = rand::thread_rng();
        let max_attempts = 5;

        let max = match difficulty {
            Difficulty::Easy => 15,
            Difficulty::Medium => 20,
            Difficulty::Hard => 50,
        };

        let range = 1..=max;

        let random_number = generate_number(&mut rng, range);

        println!("You selected {}", difficulty);
        println!("Guess the number between 1 and {}", max);
        println!("You've got {} guesses!\n", max_attempts);

        thread::sleep(Duration::from_secs(3));

        get_guess(&random_number, &max_attempts);
    }
}

fn select_difficulty() -> Difficulty {
    println!("1 - Easy | 2 - Medium | 3 - Hard");
    print!("Choose your difficulty: ");
    stdout().flush().unwrap();

    let difficulty: Difficulty;

    loop {
        let input = parse_u8();
        difficulty = match input {
            1 => Difficulty::Easy,
            2 => Difficulty::Medium,
            3 => Difficulty::Hard,
            _ => {
                println!("Error selecting difficulty, try again.");
                thread::sleep(Duration::from_secs(2));
                clear_screen();
                continue;
            }
        };
        break;
    }
    difficulty
}

fn generate_number(rng: &mut ThreadRng, range: RangeInclusive<u8>) -> u8 {
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

fn prompt() {
    thread::sleep(Duration::from_secs(2));
    println!("\nPress ENTER to play again...");
    read_input();
    clear_screen();
}

fn game_won(number: &u8) {
    clear_screen();
    println!("\nYou won!\nThe number was {}!", *number);
    prompt();
}

fn game_lost(number: &u8) {
    clear_screen();
    println!("\nYou lost :(\nThe number was {}.", *number);
    prompt();
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
