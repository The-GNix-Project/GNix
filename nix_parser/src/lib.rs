mod grammar;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use pyo3::PyClass;
use pyo3::Bound;
use pyo3::wrap_pyfunction;

use serde_json::Value;

use std::collections::HashMap;
use std::fmt;

pub use grammar::*;

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
    // Core types
    m.add_class::<Position>()?;
    m.add_class::<Span>()?;
    m.add_class::<Identifier>()?;
    m.add_class::<Error>()?;
    m.add_class::<Float>()?;
    m.add_class::<Integer>()?;

    // Operators
    m.add_class::<Addition>()?;
    m.add_class::<Concatenation>()?;
    m.add_class::<EqualTo>()?;
    m.add_class::<GreaterThan>()?;
    m.add_class::<GreaterThanOrEqualTo>()?;
    m.add_class::<Division>()?;
    m.add_class::<Implication>()?;
    m.add_class::<LessThan>()?;
    m.add_class::<LessThanOrEqualTo>()?;
    m.add_class::<LogicalAnd>()?;
    m.add_class::<LogicalOr>()?;
    m.add_class::<Multiplication>()?;
    m.add_class::<NotEqualTo>()?;
    m.add_class::<Subtraction>()?;
    m.add_class::<Update>()?;
    m.add_class::<Not>()?;
    m.add_class::<Negate>()?;

    // Function-related types
    m.add_class::<FunctionHeadDestructuredArgument>()?;
    m.add_class::<FunctionHeadDestructured>()?;
    m.add_class::<FunctionHeadSimple>()?;
    m.add_class::<Function>()?;
    m.add_class::<FunctionApplication>()?;

    // Parts
    m.add_class::<PartInterpolation>()?;
    m.add_class::<PartRaw>()?;

    // Expressions
    m.add_class::<BinaryOperation>()?;
    m.add_class::<Assert>()?;
    m.add_class::<HasAttribute>()?;
    m.add_class::<IndentedString>()?;
    m.add_class::<IfThenElse>()?;
    m.add_class::<LetIn>()?;
    m.add_class::<List>()?;
    m.add_class::<Map>()?;
    m.add_class::<Path>()?;
    m.add_class::<Uri>()?;
    m.add_class::<PropertyAccess>()?;
    m.add_class::<SearchNixPath>()?;
    m.add_class::<NixString>()?;
    m.add_class::<UnaryOperation>()?;
    m.add_class::<With>()?;

    // Bindings
    m.add_class::<BindingInherit>()?;
    m.add_class::<BindingKeyValue>()?;

    m.add_function(wrap_pyfunction!(parse_nix, m)?)?;
    m.add_function(wrap_pyfunction!(find_key_pair, m)?)?;
    Ok(())
}