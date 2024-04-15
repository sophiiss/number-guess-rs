use rand::{rngs::ThreadRng, Rng};
use std::io::{stdin, stdout, Write};

fn main() {
    let mut rng = rand::thread_rng();

    let random_number = generate_number(&mut rng);
    println!("{}, {}", parse_u8(), random_number);
}

fn generate_number(rng: &mut ThreadRng) -> u8 {
    let range: std::ops::RangeInclusive<u8> = 1..=10;
    rng.gen_range(range)
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
