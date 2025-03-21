use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use pyo3::Bound;
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

#[pyfunction]
fn find_key_pair(py: Python, node: PyObject, key: &str) -> PyResult<Option<PyObject>> {
    let bound_node = node.bind(py);

    // Check if dictionary
    if let Ok(dict) = bound_node.downcast::<PyDict>() {
        // Process KeyValue entry first
        if let Some(result) = process_keyvalue(py, &dict, key)? {
            return Ok(Some(result));
        }

        // Recursive search in dict values
        for (_, value) in dict.iter() {
            if let Some(result) = find_key_pair(py, value.to_object(py), key)? {
                return Ok(Some(result));
            }
        }
    }
    // Check if list
    else if let Ok(list) = bound_node.downcast::<PyList>() {
        // Recursive search in list items
        for item in list.iter() {
            if let Some(result) = find_key_pair(py, item.to_object(py), key)? {
                return Ok(Some(result));
            }
        }
    }

    Ok(None)
}

fn process_keyvalue(
    py: Python<'_>,
    dict: &Bound<'_, PyDict>,
    key: &str,
) -> PyResult<Option<PyObject>> {
    // Early exit if any step fails to match structure
    let Some(kv_item) = dict.get_item("KeyValue").ok().flatten() else {
        return Ok(None);
    };
    let Ok(kv_dict) = kv_item.downcast::<PyDict>() else {
        return Ok(None);
    };
    let Some(from_item) = kv_dict.get_item("from").ok().flatten() else {
        return Ok(None);
    };
    let Ok(from_list) = from_item.downcast::<PyList>() else {
        return Ok(None);
    };

    for item in from_list.iter() {
        let Ok(item_dict) = item.downcast::<PyDict>() else {
            continue;
        };
        let Some(raw_item) = item_dict.get_item("Raw").ok().flatten() else {
            continue;
        };
        let Ok(raw_dict) = raw_item.downcast::<PyDict>() else {
            continue;
        };
        let Some(content_item) = raw_dict.get_item("content").ok().flatten() else {
            continue;
        };
        let Ok(content) = content_item.extract::<String>() else {
            continue;
        };

        if content == key {
            let Some(to_item) = kv_dict.get_item("to").ok().flatten() else {
                continue;
            };
            return Ok(Some(to_item.to_object(py)));
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
fn nix_parser(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(find_key_pair, m)?)?;
    m.add_function(wrap_pyfunction!(parse_nix, m)?)?;
    m.add_function(wrap_pyfunction!(find_key_pair, m)?)?;
    Ok(())
}