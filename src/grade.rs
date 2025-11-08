use crate::hint::{HintType, WordleHint, WORD_LENGTH};

pub fn grade_guess(guess: &str, answer: &str) -> WordleHint {
    assert_eq!(guess.len(), WORD_LENGTH);
    assert_eq!(answer.len(), WORD_LENGTH);

    let mut hint_arr = [HintType::Absent; WORD_LENGTH];
    let mut unseen_pool = vec![];

    // Mark greens
    for (i, (guess_char, answer_key_char)) in guess.chars().zip(answer.chars()).enumerate() {
        if guess_char == answer_key_char {
            hint_arr[i] = HintType::Correct;
        } else {
            unseen_pool.push(answer_key_char);
        }
    }

    // Mark yellows
    for (i, guess_char) in guess.chars().enumerate() {
        if hint_arr[i] == HintType::Absent && unseen_pool.contains(&guess_char) {
            unseen_pool.remove(unseen_pool.iter().position(|c| *c == guess_char).unwrap());
            hint_arr[i] = HintType::Present;
        }
    }

    WordleHint::new(guess.to_string(), hint_arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_absent() {
        let result = grade_guess("crane", "built");
        assert_eq!(result.hints(), "XXXXX");
    }

    #[test]
    fn test_all_correct() {
        let result = grade_guess("crane", "crane");
        assert_eq!(result.hints(), "OOOOO");
    }

    #[test]
    fn test_duplicate_in_target_causes_present() {
        let result = grade_guess("roost", "robot");
        assert_eq!(result.hints(), "OO~XO");
    }

    #[test]
    fn test_guess_has_more_duplicates_than_target() {
        let result = grade_guess("allee", "apple");
        assert_eq!(result.hints(), "O~XXO");
    }

    #[test]
    fn test_no_matches() {
        let result = grade_guess("crane", "yummy");
        assert_eq!(result.hints(), "XXXXX");
    }

    #[test]
    fn test_mixed_duplicates_and_correct() {
        let result = grade_guess("ABBEY", "BANAL");
        assert_eq!(result.hints(), "~~XXX");
    }

    #[test]
    fn test_duplicate_correct_and_present_same_letter() {
        let result = grade_guess("array", "alarm");
        assert_eq!(result.hints(), "O~X~X");
    }

    #[test]
    fn test_present_does_not_steal_from_correct() {
        let result = grade_guess("babee", "aback");
        assert_eq!(result.hints(), "~~XXX");
    }
}