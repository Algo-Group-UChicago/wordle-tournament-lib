use crate::corpus::{get_corpus, is_valid_word};
use crate::hint::WordleHint;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::Bound;

const NUM_TARGET_WORDS: usize = 1000;
const DUMMY_GUESS: &str = "imagine guessing more than 5 letters";

#[pyclass(subclass)]
pub struct UChicagoWordleBotBase {
    #[pyo3(get, set)]
    team_id: String,
}

#[pymethods]
impl UChicagoWordleBotBase {
    #[new]
    pub fn new(team_id: String) -> Self {
        UChicagoWordleBotBase { team_id }
    }

    pub fn evaluate(slf: Bound<'_, Self>, grade_local: bool) -> PyResult<()> {
        let py = slf.py();
        let team_id: &str = &slf.borrow().team_id;

        // Each element of this vector is a guess history per target word that we grow via calling
        //   user's guess() method and sending guesses to backend to recieve hints.
        let hint_map: Vec<Bound<PyList>> =
            (0..NUM_TARGET_WORDS).map(|_| PyList::empty(py)).collect();

        if grade_local {
            println!("Beginning evaluation (local grading)");
        } else {
            println!("Beginning evaluation (remote grading)");
            slf.borrow().send_start_signal_to_server(team_id)?;
        }

        for _ in 0..20 {
            let mut guesses = vec![];
            for hint_list in hint_map.iter() {
                // Skip calling guess() if they've already guessed the word
                if hint_list.len() > 0 {
                    let last_hint: Bound<WordleHint> =
                        hint_list.get_item(hint_list.len() - 1)?.extract()?;
                    if last_hint.borrow().is_fully_correct() {
                        guesses.push(DUMMY_GUESS.to_string());
                        continue;
                    }
                }

                let guess: String = slf.call_method1("guess", (hint_list,))?.extract()?;
                if !is_valid_word(&guess) {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Guess {} is not a valid word - must be in corpus", guess),
                    ));
                }
                guesses.push(guess);
            }

            let new_hints = if grade_local {
                slf.borrow().grade_guesses_locally(&guesses)?
            } else {
                slf.borrow().submit_guesses_to_server(team_id, &guesses)?
            };
            
            for (i, hint_list) in hint_map.iter().enumerate() {
                let hint = Py::new(py, new_hints[i].clone())?;
                hint_list.append(hint)?;
            }
        }

        // Calculate final score
        let avg_num_guesses: f64;
        if grade_local {
            println!("Ending evaluation (local grading)");
            avg_num_guesses = Self::calculate_local_score(&hint_map, team_id)?;
        } else {
            println!("Ending evaluation (remote grading)");
            avg_num_guesses = slf.borrow().send_end_signal_to_server(team_id)?;
        }

        println!("Team {} eval completed: Avg num guesses = {:.2}", team_id, avg_num_guesses);
        Ok(())
    }

    pub fn guess(&self, _py: Python, _hints: Vec<Py<WordleHint>>) -> PyResult<String> {
        Err(PyNotImplementedError::new_err(
            "Subclass must implement the guess() method",
        ))
    }
}

impl UChicagoWordleBotBase {
    fn calculate_local_score(hint_map: &[Bound<PyList>], team_id: &str) -> Result<f64, PyErr> {
        let mut tot_guesses = 0.0;
        
        for hint_list in hint_map.iter() {
            if hint_list.len() > 0 {
                let last_hint: Bound<WordleHint> =
                    hint_list.get_item(hint_list.len() - 1)?.extract()?;
                if !last_hint.borrow().is_fully_correct() {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Team {} failed to guess some words - we will error out", team_id)
                    ));
                }
                // find number of guesses it took for the given word
                let first_correct_index = hint_list.iter()
                    .position(|hint| {
                        hint.extract::<Bound<WordleHint>>()
                            .ok()
                            .map(|h| h.borrow().is_fully_correct())
                            .unwrap_or(false)
                    })
                    .unwrap();
                tot_guesses += (first_correct_index + 1) as f64;
            }
        }
        
        Ok(tot_guesses / NUM_TARGET_WORDS as f64)
    }

    fn submit_guesses_to_server(
        &self,
        _team_id: &str,
        _guesses: &[String],
    ) -> Result<Vec<WordleHint>, PyErr> {
        todo!("Implemenet sending logic")
    }

    fn send_start_signal_to_server(&self, _team_id: &str) -> Result<(), PyErr> {
        // todo!("Implement sending start signal to server")
        // this will probably start some kind of timer
        Ok(())
    }

    fn send_end_signal_to_server(&self, _team_id: &str) -> Result<f64, PyErr> {
        // todo!("Implement sending end signal to server")
        // this will probably end some kind of timer, record the user's final score, shuffle the user's answer key for the next run etc.
        // should return the avg number of guesses
        Ok(0.0)
    }

    fn grade_guesses_locally(&self, guesses: &[String]) -> Result<Vec<WordleHint>, PyErr> {
        // todo!("Implement local grading logic")
        // For now, return dummy hints
        let mut hints = vec![];
        for guess in guesses {
            if guess == DUMMY_GUESS {
                hints.push(WordleHint::new_hint(guess.clone(), "OOOOO".to_string())?);
            } else {
                hints.push(WordleHint::new_hint(guess.clone(), "XXXXX".to_string())?);
            }
        }
        Ok(hints)
    }
}
