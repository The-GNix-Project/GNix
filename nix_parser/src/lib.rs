use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use serde_json;

/// Parses a Nix script into a JSON string.
///
/// # Arguments
/// * `nix_script` - A string containing the Nix script to parse.
///
/// # Returns
/// A JSON string representing the parsed Nix script.
///
/// # Errors
/// Returns a `PyRuntimeError` if parsing or serialization fails.
///
/// # Examples
/// ```python
/// import rust_parser
/// nix_script = "{ example = 123; }"
/// json_output = rust_parser.parse_nix_to_json(nix_script)
/// print(json_output)
/// ```
#[pyfunction]
#[pyo3(text_signature = "(nix_script: str) -> str")]
#[pyo3(name = "parse_nix")]
fn parse_nix(nix_script: String) -> PyResult<String> {
    // Directly parse the Nix script; handle any serialization errors.
    let parsed = nixel::parse(nix_script);
    serde_json::to_string(&parsed)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pymodule]
fn nix_parser(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register the function with the module
    m.add_function(wrap_pyfunction!(parse_nix, m)?)?;
    m.add("__doc__", "A Python module for parsing Nix scripts to JSON AST using Rust.")?;
    Ok(())
}