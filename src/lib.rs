use std::fs;

pub fn words_library() -> Vec<String> {
    let contents = fs::read_to_string("src/words.txt").expect("Failed to load words.txt");
    let words: Vec<String> = contents
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|word| !word.is_empty())
        .collect();

    words
}
