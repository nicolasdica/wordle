use colored::Colorize;
use rand::Rng;
use std::io;
use wordle::{check_guess, validate_guess, words_library, LetterStatus};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to Wordle!");

    let words = words_library();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    let word = words[index].clone();

    loop {
        println!("Input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match validate_guess(guess.trim(), &words) {
            Ok(_) => {}
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }

        let guess_vec: Vec<char> = guess.trim().chars().collect();
        let word_vec: Vec<char> = word.chars().collect();

        if guess_vec == word_vec {
            println!("That's it!");
            break;
        } else {
            let guess_trimmed = guess.trim();
            let results = check_guess(guess_trimmed, &word);

            for (i, status) in results.iter().enumerate() {
                match status {
                    LetterStatus::Exact => {
                        print!(" {} ", guess_vec[i].to_string().blue());
                    }
                    LetterStatus::WrongPos => {
                        print!(" {} ", guess_vec[i].to_string().red());
                    }
                    LetterStatus::NotInWord => {
                        print!(" {}", guess_vec[i]);
                    }
                }
            }

            println!()
        }
    }
}
