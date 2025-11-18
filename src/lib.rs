use std::fs;

#[derive(Debug, PartialEq)]
pub struct WordsType {
    pub letter: char,
    pub amount: i8,
    pub count: i8,
}

pub fn words_library() -> Vec<String> {
    let contents = fs::read_to_string("src/words.txt").expect("Failed to load words.txt");
    let words: Vec<String> = contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|word| !word.is_empty())
        .collect();

    words
}

pub fn build_letter_wordle_vec(word: &str) -> Vec<WordsType> {
    let word_vec: Vec<char> = word.chars().collect();
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

    wordle_vec
}

pub fn validate_guess(guess: &str, words: &[String]) -> Result<(), String> {
    if guess.len() != 5 {
        return Err("Your word guess should be of five letters".to_string());
    }

    if !words.contains(&guess.trim().to_string()) {
        return Err("That word is not in the word list!".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_words_amount() {
        let words = words_library();
        assert!(words.len() > 1);
    }

    #[test]
    fn wordle_vec_with_five_letters() {
        let wordle_vec = build_letter_wordle_vec("shine");
        assert_eq!(wordle_vec.len(), 5);

        let wordle_vec_expected = [
            WordsType { letter: 's', amount: 1, count: 0 },
            WordsType { letter: 'h', amount: 1, count: 0 },
            WordsType { letter: 'i', amount: 1, count: 0 },
            WordsType { letter: 'n', amount: 1, count: 0 },
            WordsType { letter: 'e', amount: 1, count: 0 },
        ];
        assert_eq!(wordle_vec, wordle_vec_expected);
    }

    #[test]
    fn wordle_vec_with_duplicates() {
        let wordle_vec = build_letter_wordle_vec("level");
        assert_eq!(wordle_vec.len(), 3);

        let duplcated_l = wordle_vec.iter().find(|w| w.letter == 'l');
        assert_eq!(duplcated_l.unwrap().amount, 2);

        let wordle_vec_expected = [
            WordsType { letter: 'l', amount: 2, count: 0 },
            WordsType { letter: 'e', amount: 2, count: 0 },
            WordsType { letter: 'v', amount: 1, count: 0 },
        ];
        assert_eq!(wordle_vec, wordle_vec_expected);
    }
}
