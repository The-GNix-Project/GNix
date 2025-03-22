use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::Bound;

use serde_json::Value;

pub fn json_to_py(py: Python, value: Value) -> PyObject {
    match value {
        Value::Null => py.None(),
        Value::Bool(b) => b.into_py(py),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_py(py)
            } else if let Some(f) = n.as_f64() {
                f.into_py(py)
            } else {
                py.None()
            }
        }
        Value::String(s) => s.into_py(py),
        Value::Array(arr) => {
            let py_list = PyList::empty_bound(py);
            for val in arr {
                py_list.append(json_to_py(py, val)).unwrap();
            }
            py_list.into_py(py)
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new_bound(py);
            for (key, val) in obj {
                py_dict.set_item(key, json_to_py(py, val)).unwrap();
            }
            py_dict.into_py(py)
        }
    }
}

#[pyfunction]
pub fn find_key_pair(py: Python, node: PyObject, key: &str) -> PyResult<Option<PyObject>> {
    let bound_node = node.bind(py);

    if let Ok(dict) = bound_node.downcast::<PyDict>() {
        if let Some(result) = process_keyvalue(py, &dict, key)? {
            return Ok(Some(result));
        }

        for (_, value) in dict.iter() {
            if let Some(result) = find_key_pair(py, value.into_py(py), key)? {
                return Ok(Some(result));
            }
        }
    } else if let Ok(list) = bound_node.downcast::<PyList>() {
        for item in list.iter() {
            if let Some(result) = find_key_pair(py, item.into_py(py), key)? {
                return Ok(Some(result));
            }
        }
    }

    Ok(None)
}

pub fn process_keyvalue(
    py: Python<'_>,
    dict: &Bound<'_, PyDict>,
    key: &str,
) -> PyResult<Option<PyObject>> {
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
            return Ok(Some(to_item.into_py(py)));
        }
    }

    Ok(None)
}

#[pyfunction]
pub fn parse_nix(py: Python, nix_script: String) -> PyResult<PyObject> {
    let parsed = nixel::parse(nix_script);
    let json_value = serde_json::to_value(&parsed)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let py_dict = PyDict::new_bound(py);
    if let Value::Object(map) = json_value {
        for (key, value) in map {
            py_dict.set_item(key, json_to_py(py, value))?;
        }
    }
    Ok(py_dict.into_py(py))
}