#[derive(Debug)]
enum WordProcessingError {
    EmptyWord,
    HasNumbers,
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
    let sentence = "Hello, world!";
    let words: Vec<&str> = sentence.split(" ").collect();

    let processed_results: Vec<Result<String, WordProcessingError>> =
        words.iter().map(|&word| process_word(word)).collect();

    for result in processed_results {
        match result {
            Ok(process_word) => println!("Processed {}", process_word),
            Err(WordProcessingError::EmptyWord) => println!("Error: This is an empty word"),
            Err(WordProcessingError::HasNumbers) => println!("Error: This word contains numbers")
        }
    }
}
