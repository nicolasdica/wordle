use colored::Colorize;
use rand::Rng;
use std::io;
use wordle::{build_letter_wordle_vec, validate_guess, words_library};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to Wordle!");

    let words = words_library();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    let word = words[index].clone();
    let word_vec: Vec<char> = word.chars().collect();

    loop {
        let mut wordle_vec = build_letter_wordle_vec(&word);

        println!("Input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_vec: Vec<char> = guess.trim().chars().collect();

        match validate_guess(guess.trim(), &words) {
            Ok(_) => {}
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }

        if guess_vec == word_vec {
            println!("That's it!");
            break;
        } else {
            for (index, letter) in guess_vec.iter().enumerate() {
                if guess_vec[index] == word_vec[index] {
                    print!(" {} ", guess_vec[index].to_string().blue());

                    if let Some(word_type) =
                        wordle_vec.iter_mut().find(|item| item.letter == *letter)
                    {
                        word_type.count += 1;
                    }
                } else {
                    if word_vec.contains(&guess_vec[index]) {
                        if let Some(word_type) = wordle_vec
                            .iter()
                            .find(|item| item.letter == guess_vec[index])
                        {
                            if word_type.amount > word_type.count {
                                print!(" {} ", guess_vec[index].to_string().red());
                            } else {
                                print!(" {}", guess_vec[index]);
                            }
                        } else {
                            print!(" {} ", guess_vec[index].to_string().red());
                        }
                    } else {
                        print!(" {}", guess_vec[index]);
                    }
                }
            }

            println!()
        }
    }
}
