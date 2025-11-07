use crate::hint::WordleHint;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::Bound;

const NUM_TARGET_WORDS: usize = 1000;
const DUMMY_GUESS: &str = "-----";

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

    pub fn evaluate(slf: Bound<'_, Self>) -> PyResult<()> {
        let py = slf.py();
        let team_id: &str = &slf.borrow().team_id;

        // send start signal to server
        slf.borrow().send_start_signal_to_server(team_id)?;

        // Each element of this vector is a guess history per target word that we grow via calling
        //   user's guess() method and sending guesses to backend to recieve hints.
        let hint_map: Vec<Bound<PyList>> =
            (0..NUM_TARGET_WORDS).map(|_| PyList::empty(py)).collect();

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
                if guess.len() != 5 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "Guess must be 5 letters long", // TODO: implement actual guess validation against corpus
                    ));
                }
                guesses.push(guess);
            }

            let new_hints = slf.borrow().submit_guesses_to_server(team_id, &guesses)?;
            for (i, hint_list) in hint_map.iter().enumerate() {
                let hint = Py::new(py, new_hints[i].clone())?;
                hint_list.append(hint)?;
            }
        }

        // send end signal to server
        slf.borrow().send_end_signal_to_server(team_id)?;

        println!("Team {} eval completed", team_id);
        Ok(())
    }

    pub fn guess(&self, _py: Python, _hints: Vec<Py<WordleHint>>) -> PyResult<String> {
        Err(PyNotImplementedError::new_err(
            "Subclass must implement the guess() method",
        ))
    }
}

impl UChicagoWordleBotBase {
    fn submit_guesses_to_server(
        &self,
        _team_id: &str,
        guesses: &Vec<String>,
    ) -> Result<Vec<WordleHint>, PyErr> {
        // todo!("Implemenet sending logic")

        // for now I'm gonna return dumb grading - otherwise we'd be balling w oneshot server call
        let mut hints = vec![];
        for guess in guesses {
            // The most recent hint being "OOOOO" will signal the middleware to not call guess() and use DUMMY_GUESS instead
            if guess == DUMMY_GUESS {
                hints.push(WordleHint::new_hint(guess.clone(), "OOOOO".to_string())?);
                continue;
            }
            hints.push(WordleHint::new_hint(guess.clone(), "XXXXX".to_string())?);
        }

        assert_eq!(hints.len(), guesses.len()); // turn this into an error check once server logic is implemented
        Ok(hints) // we want to still return Ok(hints) later, but the block above this will instead be [formatting -> API call -> parsing]
    }

    fn send_start_signal_to_server(&self, _team_id: &str) -> Result<(), PyErr> {
        // todo!("Implement sending start signal to server")
        // this will probably start some kind of timer
        Ok(())
    }

    fn send_end_signal_to_server(&self, _team_id: &str) -> Result<(), PyErr> {
        // todo!("Implement sending end signal to server")
        // this will probably end some kind of timer, record the user's final score, shuffle the user's answer key for the next run etc.
        Ok(())
    }
}
