use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Helper function to call Python's print function for proper Jupyter support
pub fn py_print(py: Python, msg: &str) -> PyResult<()> {
    let builtins = PyModule::import(py, "builtins")?;
    let print = builtins.getattr("print")?;
    print.call1((msg,))?;
    Ok(())
}

