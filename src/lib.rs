use pyo3::prelude::*;

mod hint;

/// Python bindings for wordle-tournament-lib
#[pymodule]
fn wordle_tournament_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<hint::WordleHint>()?;
    Ok(())
}
