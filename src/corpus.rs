use std::collections::HashSet;
use std::sync::OnceLock;

pub const WORD_LIST: &str = include_str!("../corpus.txt");

static CORPUS: OnceLock<HashSet<&'static str>> = OnceLock::new();

pub fn get_corpus() -> &'static HashSet<&'static str> {
    CORPUS.get_or_init(|| {
        WORD_LIST.lines().collect()
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
    fn test_all_words_5_letters() {
        let corpus = get_corpus();
        for word in corpus {
            assert_eq!(word.len(), 5, "Word '{}' is not 5 letters", word);
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

