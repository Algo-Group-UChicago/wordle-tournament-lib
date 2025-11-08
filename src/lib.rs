use pyo3::prelude::*;

mod base;
pub mod hint;
pub mod corpus;

/// Python bindings for wordle-tournament-lib
#[pymodule]
fn wordle_tournament_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<hint::WordleHint>()?;
    m.add_class::<base::UChicagoWordleBotBase>()?;
    Ok(())
}
