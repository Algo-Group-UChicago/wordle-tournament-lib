use crate::hint::{HintType, WordleHint, WORD_LENGTH};

pub fn grade_guess(guess: &str, answer: &str) -> WordleHint {
    assert_eq!(guess.len(), WORD_LENGTH);
    assert_eq!(answer.len(), WORD_LENGTH);

    let mut hint_arr = [HintType::Absent; WORD_LENGTH];
    for (i, (guess_char, answer_key_char)) in guess.chars().zip(answer.chars()).enumerate() {
        if guess_char == answer_key_char {
            hint_arr[i] = HintType::Correct;
        } 
    }

    WordleHint::new(guess.to_string(), hint_arr)
}