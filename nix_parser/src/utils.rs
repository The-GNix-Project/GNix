// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of GNix.
// GNix - The Graphical Nix Project
// -----------------------------------------------------------------------------------------|
// GNix is free software: you can redistribute it and/or modify                             |
// it under the terms of the GNU General Public License as published by                     |
// the Free Software Foundation, either version 3 of the License, or any later version.     |
//                                                                                          |
// GNix is distributed in the hope that it will be useful,                                  |
// but WITHOUT ANY WARRANTY; without even the implied warranty of                           |
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                            |
// GNU General Public License for more details.                                             |
//                                                                                          |
// You should have received a copy of the GNU General Public License                        |
// along with GNix.  If not, see <https://www.gnu.org/licenses/>.                           |
// -----------------------------------------------------------------------------------------|

use crate::parser::grammar::*;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::Bound;

use serde_json::Value;

pub fn build_py(py: Python, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(py.None())
            }
        }
        Value::String(s) => Ok(s.into_py(py)),
        Value::Array(arr) => {
            let py_list = PyList::empty_bound(py);
            for item in arr {
                py_list.append(build_py(py, item)?)?;
            }
            Ok(py_list.into_py(py))
        }
        Value::Object(obj) => {
            if let Some(expression) = obj.get("expression") {
                return build_py(py, expression);
            }

            let node_type = obj.get("type").and_then(Value::as_str).unwrap_or("");
        
            match node_type {
                "Position" => Ok(Position::new(
                    obj.get("line").and_then(Value::as_i64).unwrap_or(0),
                    obj.get("column").and_then(Value::as_i64).unwrap_or(0),
                ).into_py(py)),
                
                "Span" => Ok(Span::new(
                    build_py(py, obj.get("start").unwrap_or(&Value::Null))?.extract(py)?,
                    build_py(py, obj.get("end").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),
                
                "Identifier" => Ok(Identifier::new(
                    obj.get("id").and_then(Value::as_str).unwrap_or_default().to_string(),
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Error" => Ok(Error::new(
                    obj.get("message").and_then(Value::as_str).unwrap_or_default().to_string(),            
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Integer" => Ok(Integer::new(
                    obj.get("value").and_then(Value::as_str).unwrap_or_default().to_string(),
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Float" => Ok(Float::new(
                    obj.get("value").and_then(Value::as_str).unwrap_or_default().to_string(),
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Function" => Ok(Function::new(
                    build_py(py, obj.get("head").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("body").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Addition" => Ok(Addition::new().into_py(py)),
                "Concatenation" => Ok(Concatenation::new().into_py(py)),
                "EqualTo" => Ok(EqualTo::new().into_py(py)),
                "GreaterThan" => Ok(GreaterThan::new().into_py(py)),
                "GreaterThanOrEqualTo" => Ok(GreaterThanOrEqualTo::new().into_py(py)),
                "Division" => Ok(Division::new().into_py(py)),
                "Implication" => Ok(Implication::new().into_py(py)),
                "LessThan" => Ok(LessThan::new().into_py(py)),
                "LessThanOrEqualTo" => Ok(LessThanOrEqualTo::new().into_py(py)),
                "LogicalAnd" => Ok(LogicalAnd::new().into_py(py)),
                "LogicalOr" => Ok(LogicalOr::new().into_py(py)),
                "Multiplication" => Ok(Multiplication::new().into_py(py)),
                "NotEqualTo" => Ok(NotEqualTo::new().into_py(py)),
                "Subtraction" => Ok(Subtraction::new().into_py(py)),
                "Update" => Ok(Update::new().into_py(py)),
                "Not" => Ok(Not::new().into_py(py)),
                "Negate" => Ok(Negate::new().into_py(py)),

                "List" => Ok(List::new(
                    obj.get("elements").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "BinaryOperation" => Ok(BinaryOperation::new(
                    build_py(py, obj.get("left").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("operator").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("right").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "FunctionHeadDestructuredArgument" => Ok(FunctionHeadDestructuredArgument::new(
                    obj.get("identifier").and_then(Value::as_str).unwrap_or_default().to_string(),
                    obj.get("default").map(|v| build_py(py, v)).transpose()?,
                ).into_py(py)),

                "FunctionHeadDestructured" => Ok(FunctionHeadDestructured::new(
                    obj.get("ellipsis").and_then(Value::as_bool).unwrap_or(false),
                    build_py(py, obj.get("identifier").unwrap_or(&Value::Null))?.extract(py)?,
                    build_py(py, obj.get("arguments").unwrap_or(&Value::Null))?.extract(py)?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "FunctionHeadSimple" => Ok(FunctionHeadSimple::new(
                    build_py(py, obj.get("identifier").unwrap_or(&Value::Null))?.extract(py)?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "FunctionApplication" => Ok(FunctionApplication::new(
                    build_py(py, obj.get("function").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("arguments").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "PartInterpolation" => Ok(PartInterpolation::new(
                    build_py(py, obj.get("expression").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "PartRaw" => Ok(PartRaw::new(
                    obj.get("content").and_then(Value::as_str).unwrap_or_default().to_string(),
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Assert" => Ok(Assert::new(
                    build_py(py, obj.get("expression").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("target").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "HasAttribute" => Ok(HasAttribute::new(
                    build_py(py, obj.get("expression").unwrap_or(&Value::Null))?,
                    obj.get("attribute_path").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "IndentedString" => Ok(IndentedString::new(
                    obj.get("parts").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "IfThenElse" => Ok(IfThenElse::new(
                    build_py(py, obj.get("predicate").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("then").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("else_").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "LetIn" => Ok(LetIn::new(
                    obj.get("bindings").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    build_py(py, obj.get("target").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Map" => Ok(Map::new(
                    obj.get("recursive").and_then(Value::as_bool).unwrap_or(false),
                    obj.get("bindings").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Path" => Ok(Path::new(
                    obj.get("parts").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "Uri" => Ok(Uri::new(
                    obj.get("uri").and_then(Value::as_str).unwrap_or_default().to_string(),
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "PropertyAccess" => Ok(PropertyAccess::new(
                    build_py(py, obj.get("expression").unwrap_or(&Value::Null))?,
                    obj.get("attribute_path").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    obj.get("default").map(|v| build_py(py, v)).transpose()?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "SearchNixPath" => Ok(SearchNixPath::new(
                    obj.get("path").and_then(Value::as_str).unwrap_or_default().to_string(),
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "NixString" => Ok(NixString::new(
                    obj.get("parts").and_then(Value::as_array).unwrap_or(&vec![])
                        .iter()
                        .map(|v| build_py(py, v))
                        .collect::<Result<Vec<_>, _>>()?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "UnaryOperation" => Ok(UnaryOperation::new(
                    build_py(py, obj.get("operator").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("operand").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "With" => Ok(With::new(
                    build_py(py, obj.get("expression").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("target").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "BindingInherit" => Ok(BindingInherit::new(
                    obj.get("from").map(|v| build_py(py, v)).transpose()?,
                    build_py(py, obj.get("attributes").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("span").unwrap_or(&Value::Null))?.extract(py)?,
                ).into_py(py)),

                "BindingKeyValue" => Ok(BindingKeyValue::new(
                    build_py(py, obj.get("from").unwrap_or(&Value::Null))?,
                    build_py(py, obj.get("to").unwrap_or(&Value::Null))?,
                ).into_py(py)),

                _ => {
                    let dict = PyDict::new_bound(py);
                    for (k, v) in obj {
                        dict.set_item(k, build_py(py, v)?)?;
                    }
                    Ok(dict.into_py(py))
                }
            }
        }
    }
}

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
    
    let ast = build_py(py, &json_value)?;
    println!("{:?}", ast);
    Ok(ast)
}