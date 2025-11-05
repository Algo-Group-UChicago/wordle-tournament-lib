use pyo3::prelude::*;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::types::{PyList, PyTuple, PyDict};
use pyo3::Bound;
use crate::hint::WordleHint;


#[pyclass(subclass)]
pub struct UChicagoWordleBotBase {
    #[pyo3(get, set)]
    team_id: String,
}


#[pymethods]
impl UChicagoWordleBotBase {
    #[new]
    pub fn new(
        team_id: String,
    ) -> Self {
        UChicagoWordleBotBase {
            team_id,
        }
    }

    pub fn evaluate(slf: Bound<'_, Self>) -> PyResult<()> {
        let py = slf.py();
        let hints = PyList::empty(py);
        let guess_result: String = slf.call_method1("guess", (hints,))?.extract()?;

        let team_id = slf.borrow().team_id.clone();

        println!("Team {} Got guess: {}", team_id, guess_result);
        Ok(())
    }

    pub fn guess(&self, _py: Python, _hints: Vec<Py<WordleHint>>) -> PyResult<String> {
        Err(PyNotImplementedError::new_err(
            "Subclass must implement the guess() method"
        ))
    }
}

impl UChicagoWordleBotBase {
    fn send(&self) {
        todo!("Implemenet sending logic")
    }
}
