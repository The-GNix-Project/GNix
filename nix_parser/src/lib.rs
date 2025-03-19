use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use pyo3::wrap_pyfunction;
use serde_json::Value;

/// Convert a serde_json::Value into a Python object.
/// We use the `to_object(py)` method (from ToPyObject) for conversion.
fn json_to_py(py: Python, value: Value) -> PyObject {
    match value {
        Value::Null => py.None(),
        Value::Bool(b) => b.to_object(py),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_object(py)
            } else if let Some(f) = n.as_f64() {
                f.to_object(py)
            } else {
                py.None()
            }
        }
        Value::String(s) => s.to_object(py),
        Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for val in arr {
                py_list.append(json_to_py(py, val)).unwrap();
            }
            py_list.to_object(py)
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (key, val) in obj {
                py_dict.set_item(key, json_to_py(py, val)).unwrap();
            }
            py_dict.to_object(py)
        }
    }
}

/// Recursively search a Python object (a dict or list) for a key–value pair.
/// Returns an owned PyObject (if found). The function takes `node` as a PyObject
/// and immediately converts it to a &PyAny.
#[pyfunction]
fn find_key_pair(py: Python, node: PyObject, key: &str) -> PyResult<Option<PyObject>> {
    let node = node.as_ref(py);
    // If the node is a dict, try to extract it as a PyDict.
    if let Ok(dict) = node.extract::<&PyDict>() {
        if let Some(kv_item) = dict.get_item("KeyValue") {
            if let Ok(kv_dict) = kv_item.extract::<&PyDict>() {
                if let Some(from_item) = kv_dict.get_item("from") {
                    if let Ok(list) = from_item.extract::<&PyList>() {
                        if let Some(found) = list.iter().find_map(|item| {
                            if let Ok(item_dict) = item.extract::<&PyDict>() {
                                if let Some(raw_item) = item_dict.get_item("Raw") {
                                    if let Ok(raw_dict) = raw_item.extract::<&PyDict>() {
                                        if let Some(content_item) = raw_dict.get_item("content") {
                                            if let Ok(content) = content_item.extract::<String>() {
                                                if content == key {
                                                    return kv_dict.get_item("to")
                                                        .map(|v| v.to_object(py));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            None
                        }) {
                            return Ok(Some(found));
                        }
                    }
                }
            }
        }
        // Otherwise, recursively search each value in the dictionary.
        for (_, value) in dict.iter() {
            if let Some(result) = find_key_pair(py, value.to_object(py), key)? {
                return Ok(Some(result));
            }
        }
    }
    // If the node is a list, try to extract it as a PyList and search each element.
    else if let Ok(list) = node.extract::<&PyList>() {
        for item in list.iter() {
            if let Some(result) = find_key_pair(py, item.to_object(py), key)? {
                return Ok(Some(result));
            }
        }
    }
    Ok(None)
}

/// Parse a Nix script into a Python dictionary.
/// This function uses `nixel::parse` and then converts the resulting JSON to Python.
#[pyfunction]
fn parse_nix(py: Python, nix_script: String) -> PyResult<PyObject> {
    let parsed = nixel::parse(nix_script);
    let json_value = serde_json::to_value(&parsed)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    let py_dict = PyDict::new(py);
    if let Value::Object(map) = json_value {
        for (key, value) in map {
            py_dict.set_item(key, json_to_py(py, value))?;
        }
    }
    Ok(py_dict.to_object(py))
}

/// Define the Python module.
#[pymodule]
fn my_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_nix, m)?)?;
    m.add_function(wrap_pyfunction!(find_key_pair, m)?)?;
    Ok(())
}
