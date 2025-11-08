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

    // Python exposed method to grade user's guess() function on a single word
    pub fn evaluate_on_word(slf: Bound<'_, Self>, answer: String, logging: bool) -> PyResult<i64> {
        if logging {
            println!("Evaluating bot on answer: {}", answer);
            println!("----------------------------------------------------");
        }
        let py = slf.py();
        let hint_list = PyList::empty(py);
        let mut guesses = vec![];
        
        for num_guesses in 1..=MAX_GUESSES {
            let guess: String = slf.call_method1("guess", (&hint_list,))?.extract()?;
            if !is_valid_word(&guess) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Guess {} is not a valid word - must be in corpus", guess),
                ));
            }
            guesses.push(guess.clone());
            let hint = grade_guess(&guess, &answer);
            if logging {
                hint.visualize_hint()?;
            }
            if hint.is_fully_correct() {
                return Ok(num_guesses as i64);
            }
            hint_list.append(Py::new(py, hint)?)?;
        }
        
        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Failed to guess answer in {} guesses", MAX_GUESSES),
        ))
    }

    /// The big daddy method that runs tournament evaluation, either locally (only 
    /// avg guess calculation) or remotely (avg guess calculation and server grading)
    pub fn evaluate(slf: Bound<'_, Self>, grade_local: bool) -> PyResult<f64> {
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
        avg_num_guesses = Self::calculate_local_score(&hint_map)?;
        match grade_local {
            true => {
                println!("Team {} local eval completed.", team_id);
                println!("Average number of guesses (unweighted) = {:.2}", avg_num_guesses);
            }
            false => {
                println!("Ending team {} evaluation (remote grading)...", team_id);
                let score = slf.borrow().send_end_signal_to_server(team_id)?;
                println!("Team {} remote eval completed.", team_id);
                println!("Average number of guesses (unweighted) = {:.2}", avg_num_guesses);
                println!("Weighted server score = {:.2}", score);
            }
        }

        Ok(avg_num_guesses)
    }

    pub fn guess(&self, _py: Python, _hints: Vec<Py<WordleHint>>) -> PyResult<String> {
        Err(PyNotImplementedError::new_err(
            "Subclass must implement the guess() method",
        ))
    }
}

impl UChicagoWordleBotBase {

    /// Send start signal to server to start tournament evaluation - details to come
    fn send_start_signal_to_server(&self, _team_id: &str) -> Result<(), PyErr> {
        // This will probably start some kind of timer
        todo!("Implement sending start signal to server")
    }

    /// Submit a round of guesses to server and return the corresponding hints based on answer key - details to come
    fn submit_guesses_to_server(
        &self,
        _team_id: &str,
        _guesses: &[String],
    ) -> Result<Vec<WordleHint>, PyErr> {
        // For an array of guesses, this send to server and returns corresponding hints based on the answer key (stored + tracked unqiuely for each user)
        todo!("Implement guess sending logic")
    }

    /// Grade a round of guesses locally and return hints
    fn grade_guesses_locally(&self, guesses: &[String]) -> Result<Vec<WordleHint>, PyErr> {
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

    /// Send end signal to server to end tournament evaluation and return score - details to come
    fn send_end_signal_to_server(&self, _team_id: &str) -> Result<f64, PyErr> {
        // This will probably end some kind of timer, record the user's final score, shuffle the user's answer key for the next run etc.
        todo!("Implement sending end signal to server")
        // Should return the avg number of guesses
        // Ok(0.0)
    }

    /// Calculate the average number of guesses it took to guess all the words (diff from server metric)
    fn calculate_local_score(hint_map: &[Bound<PyList>]) -> Result<f64, PyErr> {
        let mut tot_guesses = 0.0;
        
        for (i, hint_list) in hint_map.iter().enumerate() {
            if hint_list.len() > 0 {
                let last_hint: Bound<WordleHint> =
                    hint_list.get_item(hint_list.len() - 1)?.extract()?;
                if !last_hint.borrow().is_fully_correct() {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Failed to guess word: {}", get_grading_answer_key()[i])
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
