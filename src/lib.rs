use pyo3::prelude::*;

pub mod hint;
mod base;

/// Python bindings for wordle-tournament-lib
#[pymodule]
fn wordle_tournament_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<hint::WordleHint>()?;
    m.add_class::<base::UChicagoWordleBotBase>()?;
    Ok(())
}
