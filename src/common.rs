pub const NUM_TARGET_WORDS: usize = 2315;
pub const WORD_LENGTH: usize = 5;
pub const MAX_GUESSES: usize = 20;
pub const DUMMY_GUESS: &str = "imagine guessing more than 5 letters";

// Eventually this will be injected at compile time
pub const API_GUESSES_ENDPOINT: &str = "http://localhost:8080/api/guesses";
