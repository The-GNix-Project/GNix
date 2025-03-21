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

// #[pyfunction]
// fn find_key_pair(py: Python, node: PyObject, key: &str) -> PyResult<Option<PyObject>> {
//     // Use the Bound API for type checking
//     let bound_node = node.bind(py);

//     // Check if dictionary
//     if let Ok(dict) = bound_node.downcast::<PyDict>() {
//         // Process KeyValue entry
//         if let Ok(Some(kv_item)) = dict.get_item("KeyValue") {
//             if let Ok(kv_dict) = kv_item.downcast::<PyDict>() {
//                 if let Ok(Some(from_item)) = kv_dict.get_item("from") {
//                     if let Ok(from_list) = from_item.downcast::<PyList>() {
//                         for item in from_list.iter() {
//                             if let Ok(item_dict) = item.downcast::<PyDict>() {
//                                 if let Ok(Some(raw_item)) = item_dict.get_item("Raw") {
//                                     if let Ok(raw_dict) = raw_item.downcast::<PyDict>() {
//                                         if let Ok(Some(content_item)) = raw_dict.get_item("content") {
//                                             if let Ok(content) = content_item.extract::<String>() {
//                                                 if content == key {
//                                                     if let Ok(Some(to_item)) = kv_dict.get_item("to") {
//                                                         return Ok(Some(to_item.to_object(py)));
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         // Recursive search in dict values
//         for (_, value) in dict.iter() {
//             if let Some(result) = find_key_pair(py, value.to_object(py), key)? {
//                 return Ok(Some(result));
//             }
//         }
//     }
//     // Check if list
//     else if let Ok(list) = bound_node.downcast::<PyList>() {
//         // Recursive search in list items
//         for item in list.iter() {
//             if let Some(result) = find_key_pair(py, item.to_object(py), key)? {
//                 return Ok(Some(result));
//             }
//         }
//     }

//     Ok(None)
// }

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
    Ok(())
}