use crate::common::{API_GUESSES_ENDPOINT, DUMMY_GUESS, MAX_GUESSES, NUM_TARGET_WORDS};
use crate::corpus::{get_grading_answer_key, is_valid_word};
use crate::grade::grade_guess;
use crate::hint::{HintType, WordleHint};
use crate::utils::py_print;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::Bound;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};


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

    /// Python exposed method to grade user's guess() function on a single word
    pub fn evaluate_on_word(slf: Bound<'_, Self>, answer: String, logging: bool) -> PyResult<i64> {
        let py = slf.py();
        
        if logging {
            py_print(py, &format!("Evaluating bot on answer: {}", answer))?;
            py_print(py, "----------------------------------------------------")?;
        }
        
        let hint_list = PyList::empty(py);
        let mut guesses = vec![];

        for num_guesses in 1..=MAX_GUESSES {
            let guess: String = slf.call_method1("guess", (&hint_list,))?.extract()?;
            if !is_valid_word(&guess) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Guess {} is not a valid word - must be in corpus",
                    guess
                )));
            }
            guesses.push(guess.clone());
            let hint = grade_guess(&guess, &answer);
            if logging {
                hint.visualize_hint(py)?;
            }
            if hint.is_fully_correct() {
                return Ok(num_guesses as i64);
            }
            hint_list.append(Py::new(py, hint)?)?;
        }

        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to guess answer in {} guesses",
            MAX_GUESSES
        )))
    }

    /// The big daddy method that runs tournament evaluation, either locally (only
    /// avg guess calculation) or remotely (avg guess calculation and server grading)
    pub fn evaluate(slf: Bound<'_, Self>, grade_local: bool) -> PyResult<f64> {
        let py = slf.py();
        let team_id: &str = &slf.borrow().team_id;

        // check for non-deterministic guess() behavior
        Self::check_deterministic_behavior(&slf)?;

        // Each element of this vector is a guess history per target word that we grow via calling
        //   user's guess() method and sending guesses to backend to recieve hints.
        let hint_map: Vec<Bound<PyList>> =
            (0..NUM_TARGET_WORDS).map(|_| PyList::empty(py)).collect();

        match grade_local {
            true => {
                py_print(py, "Beginning evaluation (local grading)")?;
            }
            false => {
                py_print(py, "Beginning evaluation (remote grading)")?;
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
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Guess {} is not a valid word - must be in corpus",
                        guess
                    )));
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
                py_print(py, &format!("Team {} local eval completed.", team_id))?;
                py_print(
                    py,
                    &format!("Average number of guesses (unweighted) = {:.2}", avg_num_guesses)
                )?;
            }
            false => {
                py_print(py, &format!("Ending team {} evaluation (remote grading)...", team_id))?;
                let score = slf.borrow().send_end_signal_to_server(team_id)?;
                py_print(py, &format!("Team {} remote eval completed.", team_id))?;
                py_print(
                    py,
                    &format!("Average number of guesses (unweighted) = {:.2}", avg_num_guesses)
                )?;
                py_print(py, &format!("Weighted server score = {:.2}", score))?;
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


// move these into a separate file?
#[derive(Serialize)]
struct GuessRequest {
    team_id: String,
    guesses: Vec<String>,
}

#[derive(Deserialize)]
struct GuessResponse {
    hints: Vec<String>,
}

impl UChicagoWordleBotBase {
    /// Check for non-deterministic guess() behavior by calling guess() multiple times
    /// with the same hint list and verifying all results are identical
    fn check_deterministic_behavior(slf: &Bound<'_, Self>) -> PyResult<()> {
        let py = slf.py();
        let mut attempts = vec![];
        for _ in 0..10 {
            let hint_list = PyList::empty(py);
            hint_list.append(Py::new(
                py,
                WordleHint::new(
                    "store".to_string(),
                    [
                        HintType::Absent,
                        HintType::Absent,
                        HintType::Absent,
                        HintType::Absent,
                        HintType::Present,
                    ],
                ),
            )?)?;
            let guess: String = slf.call_method1("guess", (hint_list,))?.extract()?;
            attempts.push(guess);
        }
        if attempts.iter().any(|g| g != &attempts[0]) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "We like determinism! But your guess() method is not deterministic. \
                 Please make it return the same guess for a given unique hint list."
            )));
        }
        Ok(())
    }

    /// Send start signal to server to start tournament evaluation - details to come
    fn send_start_signal_to_server(&self, _team_id: &str) -> Result<(), PyErr> {
        // This will probably start some kind of timer
        println!("Sending mock start signal to server");
        //todo!("Implement sending start signal to server")
        Ok(())
    }

    /// Submit a round of guesses to server and return the corresponding hints based on answer key
    fn submit_guesses_to_server(
        &self,
        team_id: &str,
        guesses: &[String],
    ) -> Result<Vec<WordleHint>, PyErr> {
        let client = Client::new();

        let request_body = GuessRequest {
            team_id: team_id.to_string(),
            guesses: guesses.to_vec(),
        };

        let response = client
            .post(API_GUESSES_ENDPOINT)
            .json(&request_body)
            .send()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to send request to server: {}", e
            )))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().unwrap_or_else(|_| "Unable to read error message".to_string());
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Server returned error status {}: {}",
                status,
                error_body
            )));
        }

        let guess_response: GuessResponse = response.json().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to parse server response: {}",
                e
            ))
        })?;

        guesses.iter()
            .zip(guess_response.hints.iter())
            .map(|(word, hint_str)| {
                if word == DUMMY_GUESS {
                    Ok(WordleHint::new_all_correct(word.clone()))
                } else {
                    WordleHint::new_hint(word.clone(), hint_str.clone())
                }
            })
            .collect::<Result<Vec<WordleHint>, PyErr>>()
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
        println!("Sending mock end signal to server");
        // todo!("Implement sending end signal to server")
        // Should return the avg number of guesses
        Ok(0.0)
    }

    /// Calculate the average number of guesses it took to guess all the words based on hint map (diff from server metric)
    fn calculate_local_score(hint_map: &[Bound<PyList>]) -> Result<f64, PyErr> {
        let mut tot_guesses = 0.0;

        for (i, hint_list) in hint_map.iter().enumerate() {
            if hint_list.len() > 0 {
                let last_hint: Bound<WordleHint> =
                    hint_list.get_item(hint_list.len() - 1)?.extract()?;
                if !last_hint.borrow().is_fully_correct() {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Failed to guess word: {}",
                        get_grading_answer_key()[i]
                    )));
                }
                // Find number of guesses it took for the given word
                let first_correct_index = hint_list
                    .iter()
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
