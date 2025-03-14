use pyo3::prelude::*;
use serde_json; // Assume your parsed AST is serializable

// Wrap the nixel parsing function
#[pyfunction]
fn parse_nix_to_json(nix_script: String) -> PyResult<String> {
    // Call nixel’s Rust parser (replace with the actual API)
    let parsed = nixel::parse(nix_script);
    
    // Serialize the parsed structure to JSON
    serde_json::to_string(&parsed)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn nixel_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_nix_to_json, m)?)?;
    Ok(())
}
