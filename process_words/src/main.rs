#[derive(Debug)]
enum WordProcessingError {
    EmptyWord,
    HasNumbers
}

fn process_word(word: &str) -> Result<String, WordProcessingError> {
    if word.is_empty() {
        return Err(WordProcessingError::EmptyWord);
    }

    if word.chars().any(|c| c.is_numeric()) {
        return Err(WordProcessingError::HasNumbers);
    }

    return Ok(word.to_uppercase());
}

fn main() {
    println!("Hello, world!");
}
