use crate::common::WORD_LENGTH;
use std::collections::HashSet;
use std::sync::OnceLock;

pub const ALL_WORDS_LIST: &str = include_str!("../word-lists/corpus.txt");
pub const ANSWER_KEY_LIST: &str = include_str!("../word-lists/possible_answers.txt");

static CORPUS: OnceLock<HashSet<&'static str>> = OnceLock::new();
static GRADING_ANSWER_KEY: OnceLock<Vec<&'static str>> = OnceLock::new();

pub fn get_corpus() -> &'static HashSet<&'static str> {
    CORPUS.get_or_init(|| ALL_WORDS_LIST.lines().collect())
}

pub fn get_grading_answer_key() -> &'static Vec<&'static str> {
    GRADING_ANSWER_KEY.get_or_init(|| {
        ANSWER_KEY_LIST
            .lines()
            .filter(|word| word.len() == WORD_LENGTH)
            .collect()
    })
}

pub fn is_valid_word(word: &str) -> bool {
    get_corpus().contains(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corpus_loads() {
        let corpus = get_corpus();
        assert!(!corpus.is_empty(), "Corpus should not be empty");
        assert!(corpus.len() > 1000, "Corpus should have many words");
    }

    #[test]
    fn test_all_words_correct_length() {
        let corpus = get_corpus();
        for word in corpus {
            assert_eq!(
                word.len(),
                WORD_LENGTH,
                "Word '{}' is not {} letters",
                word,
                WORD_LENGTH
            );
        }
    }

    #[test]
    fn test_is_valid_word() {
        assert!(is_valid_word("crane"));
        assert!(is_valid_word("hello"));
        assert!(!is_valid_word("zzzzz"));
        assert!(!is_valid_word("notinlist"));
    }

    #[test]
    fn test_case_sensitive() {
        // Assuming corpus is lowercase
        assert!(is_valid_word("crane"));
        assert!(!is_valid_word("CRANE"));
    }
}
