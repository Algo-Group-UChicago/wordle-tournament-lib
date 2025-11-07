use pyo3::prelude::*;

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

    fn to_char(self) -> char {
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
    hints: Vec<HintType>,
}

#[pymethods]
impl WordleHint {
    #[new]
    pub fn new_hint(word: String, hints: String) -> PyResult<Self> {
        if word.len() != 5 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Word must be 5 letters long",
            ));
        }
        if word.len() != hints.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Word and hints must have the same length",
            ));
        }

        let hint_types: Vec<HintType> = hints.chars().map(HintType::from_char).collect();

        Ok(WordleHint {
            word,
            hints: hint_types,
        })
    }

    #[getter]
    fn word_hint_pairs(&self) -> Vec<(char, char)> {
        self.word
            .chars()
            .zip(self.hints.iter().map(|h| h.to_char()))
            .collect()
    }

    #[getter]
    fn hints(&self) -> Vec<String> {
        self.hints.iter().map(|h| h.to_char().to_string()).collect()
    }

    fn visualize_hint(&self) -> PyResult<()> {
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
        println!("{}", output);
        Ok(())
    }

    pub fn is_fully_correct(&self) -> bool {
        self.hints.iter().all(|h| *h == HintType::Correct)
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
