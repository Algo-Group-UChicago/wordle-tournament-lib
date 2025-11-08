use crate::corpus::{get_grading_answer_key, is_valid_word};
use crate::grade::grade_guess;
use crate::hint::WordleHint;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::Bound;

const NUM_TARGET_WORDS: usize = 1000;
const MAX_GUESSES: usize = 20;
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

        match grade_local {
            true => {
                println!("Beginning evaluation (local grading)");
            }
            false => {
                println!("Beginning evaluation (remote grading)");
                slf.borrow().send_start_signal_to_server(team_id)?;
            }
        }

        for _ in 0..MAX_GUESSES {
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

            // Grade new round of guesses 
            let new_hints = match grade_local {
                true => slf.borrow().grade_guesses_locally(&guesses)?,
                false => slf.borrow().submit_guesses_to_server(team_id, &guesses)?,
            };
            
            // Update hint_map with the new hints
            for (i, hint_list) in hint_map.iter().enumerate() {
                let hint = Py::new(py, new_hints[i].clone())?;
                hint_list.append(hint)?;
            }
        }

        // Calculate final score
        let avg_num_guesses: f64;
        match grade_local {
            true => {
                println!("Ending evaluation (local grading)");
                avg_num_guesses = Self::calculate_local_score(&hint_map, team_id)?;
            }
            false => {
                println!("Ending evaluation (remote grading)");
                avg_num_guesses = slf.borrow().send_end_signal_to_server(team_id)?;
            }
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

    fn send_start_signal_to_server(&self, _team_id: &str) -> Result<(), PyErr> {
        // This will probably start some kind of timer
        todo!("Implement sending start signal to server")
    }

    fn submit_guesses_to_server(
        &self,
        _team_id: &str,
        _guesses: &[String],
    ) -> Result<Vec<WordleHint>, PyErr> {
        // For an array of guesses, this send to server and returns corresponding hints based on the answer key (stored + tracked unqiuely for each user)
        todo!("Implement guess sending logic")
    }

    fn grade_guesses_locally(&self, guesses: &[String]) -> Result<Vec<WordleHint>, PyErr> {
        // This is the local versoin of submit_guesses_to_server()
        let answer_key = get_grading_answer_key();

        let mut hints = vec![];
        for (i, guess) in guesses.iter().enumerate() {
            let hint;
            if guess == DUMMY_GUESS {
                hint = WordleHint::new_all_correct(guess.clone());
            } else {
                hint = grade_guess(guess, answer_key[i]);
            }
            hints.push(hint);
        }
        Ok(hints)
    }

    fn send_end_signal_to_server(&self, _team_id: &str) -> Result<f64, PyErr> {
        // This will probably end some kind of timer, record the user's final score, shuffle the user's answer key for the next run etc.
        todo!("Implement sending end signal to server")
        // Should return the avg number of guesses
        // Ok(0.0)
    }

    fn calculate_local_score(hint_map: &[Bound<PyList>], team_id: &str) -> Result<f64, PyErr> {
        // This is the local version of send_end_signal_to_server()
        let mut tot_guesses = 0.0;
        
        for (i, hint_list) in hint_map.iter().enumerate() {
            if hint_list.len() > 0 {
                let last_hint: Bound<WordleHint> =
                    hint_list.get_item(hint_list.len() - 1)?.extract()?;
                if !last_hint.borrow().is_fully_correct() {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Team {} failed to guess word: {}", team_id, get_grading_answer_key()[i])
                    ));
                }
                // Find number of guesses it took for the given word
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
}
