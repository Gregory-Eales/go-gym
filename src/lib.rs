use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn go_gym(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(example_function, m)?)?;
    Ok(())
}

#[pyfunction]
fn example_function() -> PyResult<String> {
    Ok("Hello from Rust!".to_string())
}


