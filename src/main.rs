use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::exit;
#[derive(Default)]
struct Guess {
    guess: String,
    result: [i8; 5],
}

#[derive(Default)]
struct Game {
    target: String,
    turn: usize,
}

fn get_random_word() -> io::Result<String> {
    let file: File = File::open("words.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut rng = thread_rng();

    if let Some(random_line) = lines.choose(&mut rng) {
        Ok(random_line.clone())
    } else {
        Ok("ERROR".to_string())
    }
}

fn get_guess() -> String {
    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: String = input.trim().to_string();

        if input.len() == 5 && input.chars().all(|c| c.is_alphabetic()) {
            return input;
        } else {
            println!("Invalid input. Please enter exactly 5 letters.");
        }
    }
}

//TODO: don't allow multiple same character to appear yellow more than once if it only exists once
fn evaluate_guess(guess: &str, target: &str) -> [i8; 5] {
    let mut arr: [i8; 5] = [-1, -1, -1, -1, -1];
    for (i, c) in guess.chars().enumerate() {
        if target.contains(c) {
            if c == target.chars().nth(i).unwrap() {
                arr[i] = 1;
            } else {
                arr[i] = 0;
            }
        }
    }
    arr
}

fn is_game_won(score: [i8; 5]) -> bool {
    for i in score {
        if i != 1 {
            return false;
        }
    }
    return true;
}

fn print_guess(guess: &str, score: [i8; 5]) {
    for (i, &s) in score.iter().enumerate() {
        match s {
            -1 => print!("{}", guess.chars().nth(i).unwrap().to_string().red()),
            0 => print!("{}", guess.chars().nth(i).unwrap().to_string().yellow()),
            1 => print!("{}", guess.chars().nth(i).unwrap().to_string().green()),
            _ => unreachable!(),
        }
    }
    println!("");
}

fn main() {
    let mut game: Game = Default::default();
    let mut guesses: [Guess; 6] = Default::default();

    match get_random_word() {
        Ok(word) => game.target = word,
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    println!("PSST.. {}", game.target);

    while game.turn < 5 {
        if game.turn != 0 {
            let mut i = 0;
            for guess in &guesses {
                if i >= game.turn {
                    break;
                }
                print_guess(&guess.guess, guess.result);
                i += 1;
            }
        }
        let guess: String = get_guess();
        let guess_score: [i8; 5] = evaluate_guess(&guess, &game.target);

        guesses[game.turn] = Guess {
            guess,
            result: guess_score,
        };
        let is_game_won = is_game_won(guess_score);
        if is_game_won {
            for guess in &guesses {
                print_guess(&guess.guess, guess_score);
            }
            println!("YOU WIN");
            exit(1)
        }
        game.turn += 1;
    }
    println!("GAME OVER");
    println!("WORD IS: {}", game.target);
    exit(0)
}
