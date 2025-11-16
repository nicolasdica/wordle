use colored::Colorize;
use rand::Rng;
use std::io;
use wordle::words_library;

#[derive(Debug)]
struct WordsType {
    letter: char,
    amount: i8,
    count: i8,
}

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to Wordle!");

    let words = words_library();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    let word = words[index].clone();
    let word_vec: Vec<char> = word.chars().collect();

    loop {
        let mut wordle_vec: Vec<WordsType> = Vec::new();
        for letter in word_vec.iter() {
            if let Some(word_type) = wordle_vec.iter_mut().find(|item| item.letter == *letter) {
                word_type.amount += 1;
            } else {
                wordle_vec.push(WordsType {
                    letter: *letter,
                    amount: 1,
                    count: 0,
                });
            }
        }

        println!("Input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_vec: Vec<char> = guess.trim().chars().collect();

        if guess_vec.len() != 5 {
            println!("Your word guess should be of five letters");
            continue;
        }

        if !words.contains(&guess.trim().to_string()) {
            println!("That word is not in the word list!");
            continue;
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
