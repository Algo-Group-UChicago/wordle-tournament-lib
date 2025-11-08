use pyo3::prelude::*;

mod wordle_bot_base;
pub mod hint;
pub mod corpus;
pub mod grade;

/// Python bindings for wordle-tournament-lib
#[pymodule]
fn wordle_tournament_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<hint::WordleHint>()?;
    m.add_class::<wordle_bot_base::UChicagoWordleBotBase>()?;
    m.add_function(pyo3::wrap_pyfunction!(grade::grade_guess_py, m)?)?;
    Ok(())
}
