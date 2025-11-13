use pyo3::prelude::*;
use crate::utils::py_print;

pub const WORD_LENGTH: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintType {
    Correct,
    Present,
    Absent,
}

impl HintType {
    fn from_char(value: char) -> Self {
        match value {
            'O' => HintType::Correct,
            '~' => HintType::Present,
            'X' => HintType::Absent,
            _ => {
                unreachable!("Invalid hint type: '{}'. Must be 'O' (correct), '~' (misplaced), or 'X' (absent)", value)
            }
        }
    }

    pub fn to_char(self) -> char {
        match self {
            HintType::Correct => 'O',
            HintType::Present => '~',
            HintType::Absent => 'X',
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct WordleHint {
    #[pyo3(get)]
    word: String,
    hints: [HintType; WORD_LENGTH],
}

#[pymethods]
impl WordleHint {
    #[new]
    pub fn new_hint(word: String, hints: String) -> PyResult<Self> {
        if word.len() != WORD_LENGTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Word must be {} letters long",
                WORD_LENGTH
            )));
        }
        if word.len() != hints.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Word and hints must have the same length",
            ));
        }

        let hint_vec: Vec<HintType> = hints.chars().map(HintType::from_char).collect();
        let hint_array: [HintType; WORD_LENGTH] = hint_vec.try_into().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Expected exactly {} hints",
                WORD_LENGTH
            ))
        })?;

        Ok(WordleHint::new(word, hint_array))
    }

    #[getter]
    fn word_hint_pairs(&self) -> Vec<(char, char)> {
        self.word
            .chars()
            .zip(self.hints.iter().map(|h| h.to_char()))
            .collect()
    }

    #[getter]
    pub fn hints(&self) -> String {
        self.hints.iter().map(|h| h.to_char()).collect()
    }

    pub fn visualize_hint(&self, py: Python) -> PyResult<()> {
        let mut letters = Vec::new();
        let mut squares = Vec::new();

        for (letter, hint_type) in self.word.chars().zip(self.hints.iter()) {
            letters.push(letter.to_uppercase().to_string());
            match hint_type {
                HintType::Correct => squares.push("🟩"),
                HintType::Present => squares.push("🟨"),
                HintType::Absent => squares.push("⬜"),
            }
        }

        let output = format!("{}\n{}", letters.join(" "), squares.join(" "));
        py_print(py, &output)?;
        
        Ok(())
    }

    fn __repr__(&self) -> String {
        let hint_chars: Vec<String> = self
            .hints
            .iter()
            .map(|h| format!("'{}'", h.to_char()))
            .collect();
        format!(
            "WordleHint(word='{}', hints=[{}])",
            self.word,
            hint_chars.join(", ")
        )
    }
}

impl WordleHint {
    pub fn new(word: String, hints: [HintType; WORD_LENGTH]) -> Self {
        WordleHint { word, hints }
    }

    pub fn new_all_correct(word: String) -> Self {
        WordleHint::new(word, [HintType::Correct; WORD_LENGTH])
    }

    pub fn is_fully_correct(&self) -> bool {
        self.hints.iter().all(|h| *h == HintType::Correct)
    }
}
