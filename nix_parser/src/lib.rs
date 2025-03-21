use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use pyo3::PyClass;
use serde_json::{Map, Value};
use std::collections::HashMap;
use pyo3::Bound;
use pyo3::wrap_pyfunction;


// ==================== CORE STRUCTURES =================
#[pyclass]
#[derive(Clone)]
struct Position {
    #[pyo3(get)]
    line: i64,
    #[pyo3(get)]
    column: i64,
}

#[pymethods]
impl Position {
    #[new]
    fn new(line: i64, column: i64) -> Self { Self { line, column } }
    fn __repr__(&self) -> String { format!("Position({}, {})", self.line, self.column) }
}

#[pyclass]
#[derive(Clone)]
struct Span {
    #[pyo3(get)]
    start: Position,
    #[pyo3(get)]
    end: Position,
}

#[pymethods]
impl Span {
    #[new]
    fn new(start: Position, end: Position) -> Self { Self { start, end } }
    fn __repr__(&self) -> String { format!("Span({} to {})", self.start, self.end) }
}

#[pyclass]
#[derive(Clone)]
struct Identifier {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Identifier {
    #[new]
    fn new(id: String, span: Span) -> Self { Self { id, span } }
    fn __repr__(&self) -> String { format!("Identifier('{}')", self.id) }
}

#[pyclass]
#[derive(Clone)]
struct Error {
    #[pyo3(get)]
    message: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Error {
    #[new]
    fn new(message: String, span: Span) -> Self {
        Error { message, span }
    }

    fn __repr__(&self) -> String {
        format!("Error('{}')", self.message)
    }
}

#[pyclass]
#[derive(Clone)]
struct Float {
    #[pyo3(get)]
    value: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Float {
    #[new]
    fn new(value: String, span: Span) -> Self {
        Float { value, span }
    }

    fn __repr__(&self) -> String {
        format!("Float('{}')", self.value)
    }
}

// ==================== OPERATORS ====================
macro_rules! impl_operator {
    ($($name:ident),+) => {
        $(
            #[pyclass]
            #[derive(Clone, Copy)]
            struct $name;
            #[pymethods]
            impl $name {
                #[new] fn new() -> Self { Self }
                #[classattr] fn value() -> &'static str { stringify!($name) }
                fn __repr__(&self) -> String { format!("{}", stringify!($name)) }
            }
        )+
    };
}

impl_operator!(
    Addition, Concatenation, EqualTo, GreaterThan, GreaterThanOrEqualTo, Division,
    Implication, LessThan, LessThanOrEqualTo, LogicalAnd, LogicalOr, Multiplication,
    NotEqualTo, Subtraction, Update, Not, Negate
);

// ==================== FUNCTION STRUCTURES ====================
#[pyclass]
#[derive(Clone)]
struct FunctionHeadDestructuredArgument {
    #[pyo3(get)]
    identifier: String,
    #[pyo3(get)]
    default: Option<PyObject>,
}

#[pymethods]
impl FunctionHeadDestructuredArgument {
    #[new]
    fn new(identifier: String, default: Option<PyObject>) -> Self {
        FunctionHeadDestructuredArgument { identifier, default }
    }

    fn __repr__(&self) -> String {
        format!("FunctionHeadDestructuredArgument(identifier='{}', default={:?})",
            self.identifier, self.default)
    }
}

#[pyclass]
#[derive(Clone)]
struct FunctionHeadDestructured {
    #[pyo3(get)]
    ellipsis: bool,
    #[pyo3(get)]
    identifier: Identifier,
    #[pyo3(get)]
    arguments: FunctionHeadDestructuredArgument,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl FunctionHeadDestructured {
    #[new]
    fn new(ellipsis: bool, identifier: Identifier, arguments: FunctionHeadDestructuredArgument, span: Span) -> Self {
        Self { ellipsis, identifier, arguments, span }
    }
    fn __repr__(&self) -> String {
        format!("FunctionHeadDestructured(ellipsis={})", self.ellipsis)
    }
}

#[pyclass]
#[derive(Clone)]
struct FunctionHeadSimple {
    #[pyo3(get)]
    identifier: Identifier,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl FunctionHeadSimple {
    #[new]
    fn new(identifier: Identifier, span: Span) -> Self {
        FunctionHeadSimple { identifier, span }
    }

    fn __repr__(&self) -> String {
        format!("FunctionHeadSimple(identifier={}, span={})", self.identifier, self.span)
    }
}

#[pyclass]
#[derive(Clone)]
struct Function {
    #[pyo3(get)]
    head: PyObject,
    #[pyo3(get)]
    body: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pyclass]
#[derive(Clone)]
struct FunctionApplication {
    #[pyo3(get)]
    function: PyObject,
    #[pyo3(get)]
    arguments: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl FunctionApplication {
    #[new]
    fn new(function: PyObject, arguments: PyObject, span: Span) -> Self {
        FunctionApplication { function, arguments, span }
    }

    fn __repr__(&self) -> String {
        format!("FunctionApplication({:?})", self.function)
    }
}

#[pymethods]
impl Function {
    #[new]
    fn new(head: PyObject, body: PyObject, span: Span) -> Self {
        Function { head, body, span }
    }
    fn __repr__(&self) -> String {
        format!("Function(head={:?})", self.head)
    }
}

// ==================== PARTS ====================
#[pyclass]
#[derive(Clone)]
struct PartInterpolation {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl PartInterpolation {
    #[new]
    fn new(expression: PyObject, span: Span) -> Self {
        PartInterpolation { expression, span }
    }
    fn __repr__(&self) -> String {
        format!("PartInterpolation({:?})", self.expression)
    }
}

#[pyclass]
#[derive(Clone)]
struct PartRaw {
    #[pyo3(get)]
    content: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl PartRaw {
    #[new]
    fn new(content: String, span: Span) -> Self {
        PartRaw { content, span }
    }
    fn __repr__(&self) -> String {
        format!("PartRaw('{}')", self.content)
    }
}

// ==================== EXPRESSIONS ====================
#[pyclass]
#[derive(Clone)]
struct BinaryOperation {
    #[pyo3(get)]
    left: PyObject,
    #[pyo3(get)]
    operator: PyObject,
    #[pyo3(get)]
    right: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl BinaryOperation {
    #[new]
    fn new(left: PyObject, operator: PyObject, right: PyObject, span: Span) -> Self {
        Self { left, operator, right, span }
    }
    fn __repr__(&self) -> String {
        format!("BinaryOperation({:?}, {:?}, {:?})", self.left, self.operator, self.right)
    }
}

#[pyclass]
#[derive(Clone)]
struct Assert {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    target: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Assert {
    #[new]
    fn new(expression: PyObject, target: PyObject, span: Span) -> Self {
        Self { expression, target, span }
    }
    fn __repr__(&self) -> String {
        format!("Assert(expr={:?}, target={:?})", self.expression, self.target)
    }
}

#[pyclass]
#[derive(Clone)]
struct HasAttribute {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    attribute_path: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl HasAttribute {
    #[new]
    fn new(expression: PyObject, attribute_path: Vec<PyObject>, span: Span) -> Self {
        HasAttribute { expression, attribute_path, span }
    }
    fn __repr__(&self) -> String {
        format!("HasAttribute({} attributes)", self.attribute_path.len())
    }
}

#[pyclass]
#[derive(Clone)]
struct IndentedString {
    #[pyo3(get)]
    parts: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl IndentedString {
    #[new]
    fn new(parts: Vec<PyObject>, span: Span) -> Self {
        IndentedString { parts, span }
    }
    fn __repr__(&self) -> String {
        format!("IndentedString({} parts)", self.parts.len())
    }
}

#[pyclass]
#[derive(Clone)]
struct Integer {
    #[pyo3(get)]
    value: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Integer {
    #[new]
    fn new(value: String, span: Span) -> Self {
        Integer { value, span }
    }
    fn __repr__(&self) -> String {
        format!("Integer('{}')", self.value)
    }
}

#[pyclass]
#[derive(Clone)]
struct IfThenElse {
        #[pyo3(get)]
        predicate: PyObject,
        #[pyo3(get)]
        then: PyObject,
        #[pyo3(get)]
        else_: PyObject,
        #[pyo3(get)]
        span: Span,
    }

    #[pymethods]
    impl IfThenElse {
        #[new]
        fn new(predicate: PyObject, then: PyObject, else_: PyObject, span: Span) -> Self {
            Self { predicate, then, else_, span }
        }
        fn __repr__(&self) -> String { format!("IfThenElse(...)") }
    }

#[pyclass]
#[derive(Clone)]
struct LetIn {
        #[pyo3(get)]
        bindings: Vec<PyObject>,  // Binding objects
        #[pyo3(get)]
        target: PyObject,
        #[pyo3(get)]
        span: Span,
    }

    #[pymethods]
    impl LetIn {
        #[new]
        fn new(bindings: Vec<PyObject>, target: PyObject, span: Span) -> Self {
            Self { bindings, target, span }
        }
        fn __repr__(&self) -> String { format!("LetIn({} bindings)", self.bindings.len()) }
    }

// ==================== COLLECTIONS ====================
#[pyclass]
#[derive(Clone)]
struct List {
    #[pyo3(get)]
    elements: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl List {
    #[new]
    fn new(elements: Vec<PyObject>, span: Span) -> Self {
        List { elements, span }
    }
    fn __repr__(&self) -> String {
        format!("List({} items)", self.elements.len())
    }
}

#[pyclass]
#[derive(Clone)]
struct Map {
    #[pyo3(get)]
    recursive: bool,
    #[pyo3(get)]
    bindings: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Map {
    #[new]
    fn new(recursive: bool, bindings: Vec<PyObject>, span: Span) -> Self {
        Map { recursive, bindings, span }
    }
    fn __repr__(&self) -> String {
        format!("Map(recursive={}, {} bindings)", self.recursive, self.bindings.len())
    }
}

// ==================== PATH & URI ====================
#[pyclass]
#[derive(Clone)]
struct Path {
    #[pyo3(get)]
    parts: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Path {
    #[new]
    fn new(parts: Vec<PyObject>, span: Span) -> Self {
        Path { parts, span }
    }
    fn __repr__(&self) -> String {
        format!("Path({} parts)", self.parts.len())
    }
}

#[pyclass]
#[derive(Clone)]
struct Uri {
    #[pyo3(get)]
    uri: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Uri {
    #[new]
    fn new(uri: String, span: Span) -> Self {
        Uri { uri, span }
    }
    fn __repr__(&self) -> String {
        format!("Uri('{}')", self.uri)
    }
}

// ==================== PROPERTY ACCESS ====================
#[pyclass]
#[derive(Clone)]
struct PropertyAccess {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    attribute_path: Vec<PyObject>,
    #[pyo3(get)]
    default: Option<PyObject>,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl PropertyAccess {
    #[new]
    fn new(expression: PyObject, attribute_path: Vec<PyObject>, default: Option<PyObject>, span: Span) -> Self {
        PropertyAccess { expression, attribute_path, default, span }
    }
    fn __repr__(&self) -> String {
        format!("PropertyAccess({} attributes)", self.attribute_path.len())
    }
}

// ==================== REMAINING TYPES ====================
#[pyclass]
#[derive(Clone)]
struct SearchNixPath {
    #[pyo3(get)]
    path: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl SearchNixPath {
    #[new]
    fn new(path: String, span: Span) -> Self {
        SearchNixPath { path, span }
    }
    fn __repr__(&self) -> String {
        format!("SearchNixPath('{}')", self.path)
    }
}

#[pyclass]
#[derive(Clone)]
struct NixString {
    #[pyo3(get)]
    parts: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl NixString {
    #[new]
    fn new(parts: Vec<PyObject>, span: Span) -> Self {
        NixString { parts, span }
    }
    fn __repr__(&self) -> String {
        format!("String({} parts)", self.parts.len())
    }
}

#[pyclass]
#[derive(Clone)]
struct UnaryOperation {
    #[pyo3(get)]
    operator: PyObject,
    #[pyo3(get)]
    operand: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl UnaryOperation {
    #[new]
    fn new(operator: PyObject, operand: PyObject, span: Span) -> Self {
        UnaryOperation { operator, operand, span }
    }
    fn __repr__(&self) -> String {
        format!("UnaryOperation({:?})", self.operator)
    }
}

#[pyclass]
#[derive(Clone)]
struct With {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    target: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl With {
    #[new]
    fn new(expression: PyObject, target: PyObject, span: Span) -> Self {
        With { expression, target, span }
    }
    fn __repr__(&self) -> String {
        format!("With({:?})", self.expression)
    }
}

// ==================== BINDINGS ====================
#[pyclass]
#[derive(Clone)]
struct BindingInherit {
    #[pyo3(get)]
    from_: Option<PyObject>,
    #[pyo3(get)]
    attributes: PyObject,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl BindingInherit {
    #[new]
    fn new(from_: Option<PyObject>, attributes: PyObject, span: Span) -> Self {
        BindingInherit { from_, attributes, span }
    }
    fn __repr__(&self) -> String {
        format!("BindingInherit(from={:?})", self.from_.is_some())
    }
}

#[pyclass]
#[derive(Clone)]
struct BindingKeyValue {
    #[pyo3(get)]
    from_: PyObject,  // Part object
    #[pyo3(get)]
    to: PyObject,
}

#[pymethods]
impl BindingKeyValue {
    #[new]
    fn new(from_: PyObject, to: PyObject) -> Self { Self { from_, to } }
    fn __repr__(&self) -> String { format!("KeyValue({:?})", self.from_) }
}

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

#[pymodule]
fn nix_parser(py: Python, m: &PyModule) -> PyResult<()> {
    // Register all classes
    m.add_class::<Position>()?;
    m.add_class::<Span>()?;
    m.add_class::<Identifier>()?;
    m.add_class::<BinaryOperation>()?;
    m.add_class::<IfThenElse>()?;
    
    // Register operators
    m.add_class::<Addition>()?;
    m.add_class::<Concatenation>()?;
    // Add other operators...
    
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