use pyo3::prelude::*;

pub mod backend_client;
pub mod common;
pub mod corpus;
pub mod grade;
pub mod hint;
pub mod utils;
mod wordle_bot_base;

/// Python bindings for wordle-tournament-lib
#[pymodule]
fn wordle_tournament_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<hint::WordleHint>()?;
    m.add_class::<wordle_bot_base::UChicagoWordleBotBase>()?;
    // Avoiding exposure of grade_guess for now because it makes things too easy
    // m.add_function(pyo3::wrap_pyfunction!(grade::grade_guess_py, m)?)?;
    Ok(())
}
