use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::PyClass;

use std::fmt;

// ==================== CORE STRUCTURES =================
// MARK: Position
#[pyclass]
#[derive(Clone)]
pub struct Position {
    #[pyo3(get)]
    line: i64,
    #[pyo3(get)]
    column: i64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position({}, {})", self.line, self.column)
    }
}

#[pymethods]
impl Position {
    #[new]
    fn new(line: i64, column: i64) -> Self { Self { line, column } }
    fn __repr__(&self) -> String { format!("{}", self) }
}
// MARK: Span
#[pyclass]
#[derive(Clone)]
pub struct Span {
    #[pyo3(get)]
    start: Position,
    #[pyo3(get)]
    end: Position,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Span({}, {})", self.start, self.end)
    }
}

#[pymethods]
impl Span {
    #[new]
    fn new(start: Position, end: Position) -> Self { Self { start, end } }
    fn __repr__(&self) -> String { format!("{}", self) }
}
// MARK: Identifier
#[pyclass]
#[derive(Clone)]
pub struct Identifier {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    span: Span,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier({})", self.id)
    }
}

#[pymethods]
impl Identifier {
    #[new]
    fn new(id: String, span: Span) -> Self { Self { id, span } }
    fn __repr__(&self) -> String { format!("{}", self.id) }
}
// MARK: ERROR
#[pyclass]
#[derive(Clone)]
pub struct Error {
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
// MARK: Float
#[pyclass]
#[derive(Clone)]
pub struct Float {
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
// MARK: Integer
#[pyclass]
#[derive(Clone)]
pub struct Integer {
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

// ==================== OPERATORS ====================
// MARK: OPERATORS
macro_rules! impl_operator {
    ($($name:ident),+) => {
        $(
            #[pyclass]
            #[derive(Clone, Copy)]
            pub struct $name;
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
// MARK: FunctionHeadDestructuredArgument
#[pyclass]
pub struct FunctionHeadDestructuredArgument {
    #[pyo3(get)]
    identifier: String,
    #[pyo3(get)]
    default: Option<PyObject>,
}

impl Clone for FunctionHeadDestructuredArgument {
    fn clone(&self) -> Self {
        // Acquire the GIL token
        Python::with_gil(|py| {
            Self {
                identifier: self.identifier.clone(),
                default: self.default.as_ref().map(|py_any| py_any.clone_ref(py)),  // Pass `py` to clone_ref
            }
        })
    }
}

#[pymethods]
impl FunctionHeadDestructuredArgument {
    #[new]
    fn new(identifier: String, default: Option<PyObject>) -> Self {
        FunctionHeadDestructuredArgument { identifier, default }
    }

    fn __repr__(&self) -> String {
        format!(
            "FunctionHeadDestructuredArgument(identifier='{}', default={:?})",
            self.identifier, self.default
        )
    }
}
// MARK: FunctionHeadDestructured
#[pyclass]
#[derive(Clone)]
pub struct FunctionHeadDestructured {
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
// MARK: FunctionHeadSimple
#[pyclass]
#[derive(Clone)]
pub struct FunctionHeadSimple {
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
// MARK: Function
#[pyclass]
pub struct Function {
    #[pyo3(get)]
    head: PyObject,
    #[pyo3(get)]
    body: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                head: self.head.clone_ref(py),  // Use clone_ref with the GIL token
                body: self.body.clone_ref(py),  // Use clone_ref with the GIL token
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl Function {
    #[new]
    fn new(head: PyObject, body: PyObject, span: Span) -> Self {
        Function { head, body, span }
    }
    fn __repr__(&self) -> String {
        format!("Function({:?}, {:?})", self.head, self.body)
    }
}

// MARK: FunctionApplication
#[pyclass] 
pub struct FunctionApplication {
    #[pyo3(get)]
    function: PyObject,  
    #[pyo3(get)]
    arguments: PyObject,  
    #[pyo3(get)]
    span: Span,
}

impl Clone for FunctionApplication {
    fn clone(&self) -> Self {
        // Acquire the GIL token
        Python::with_gil(|py| {
            Self {
                function: self.function.clone_ref(py), 
                arguments: self.arguments.clone_ref(py),  
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl FunctionApplication {
    #[new]
    fn new(function: PyObject, arguments: PyObject, span: Span) -> Self {
        FunctionApplication {
            function,
            arguments,
            span,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "FunctionApplication(function={:?}, arguments={:?})",
            self.function, self.arguments
        )
    }
}

// ==================== PARTS ====================
// MARK: PartInterpolation
#[pyclass]
pub struct PartInterpolation {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for PartInterpolation {
    fn clone(&self) -> Self {
        // Acquire the GIL token
        Python::with_gil(|py| {
            Self {
                expression: self.expression.clone_ref(py), 
                span: self.span.clone()
            }
        })
    }
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
// MARK: PartRaw
#[pyclass]
#[derive(Clone)]
pub struct PartRaw {
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
// MARK: BinaryOperation
#[pyclass]
pub struct BinaryOperation {
    #[pyo3(get)]
    left: PyObject,
    #[pyo3(get)]
    operator: PyObject,
    #[pyo3(get)]
    right: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for BinaryOperation {
    fn clone(&self) -> Self {
        // Acquire the GIL token
        Python::with_gil(|py| {
            Self {
                left: self.left.clone_ref(py),  
                operator: self.operator.clone_ref(py),  
                right: self.operator.clone_ref(py),
                span: self.span.clone(),
            }
        })
    }
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
// MARK: Assert
#[pyclass]
pub struct Assert {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    target: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for Assert {
    fn clone(&self) -> Self {
        // Acquire the GIL token
        Python::with_gil(|py| {
            Self {
                expression: self.expression.clone_ref(py),  
                target: self.target.clone_ref(py),  
                span: self.span.clone(),
            }
        })
    }
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

// ==================== EXPRESSIONS ====================
// MARK: HasAttribute
#[pyclass]
pub struct HasAttribute {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    attribute_path: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

impl Clone for HasAttribute {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                expression: self.expression.clone_ref(py),
                attribute_path: self.attribute_path.iter().map(|x| x.clone_ref(py)).collect(),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl HasAttribute {
    #[new]
    fn new(expression: PyObject, attribute_path: Vec<PyObject>, span: Span) -> Self {
        HasAttribute { expression, attribute_path, span }
    }
    fn __repr__(&self) -> String {
        format!("HasAttribute({:?})", self.attribute_path)
    }
}
// MARK: IndentedString
#[pyclass]
pub struct IndentedString {
    #[pyo3(get)]
    parts: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

impl Clone for IndentedString {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                parts: self.parts.iter().map(|x| x.clone_ref(py)).collect(),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl IndentedString {
    #[new]
    fn new(parts: Vec<PyObject>, span: Span) -> Self {
        IndentedString { parts, span }
    }
    fn __repr__(&self) -> String {
        format!("IndentedString({:?})", self.parts)
    }
}
// MARK: IfThenElse
#[pyclass]
pub struct IfThenElse {
    #[pyo3(get)]
    predicate: PyObject,
    #[pyo3(get)]
    then: PyObject,
    #[pyo3(get)]
    else_: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for IfThenElse {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                predicate: self.predicate.clone_ref(py),
                then: self.then.clone_ref(py),
                else_: self.else_.clone_ref(py),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl IfThenElse {
    #[new]
    fn new(predicate: PyObject, then: PyObject, else_: PyObject, span: Span) -> Self {
        Self { predicate, then, else_, span }
    }
    fn __repr__(&self) -> String { format!("IfThenElse({}, {}, {})", self.predicate, self.then, self.else_) }
}
// MARK: LetIn
#[pyclass]
pub struct LetIn {
    #[pyo3(get)]
    bindings: Vec<PyObject>,
    #[pyo3(get)]
    target: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for LetIn {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                bindings: self.bindings.iter().map(|x| x.clone_ref(py)).collect(),
                target: self.target.clone_ref(py),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl LetIn {
    #[new]
    fn new(bindings: Vec<PyObject>, target: PyObject, span: Span) -> Self {
        Self { bindings, target, span }
    }
    fn __repr__(&self) -> String { format!("LetIn({:?})", self.bindings) }
}

// ==================== COLLECTIONS ====================
// MARK: List
#[pyclass]
pub struct List {
    #[pyo3(get)]
    elements: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

impl Clone for List {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                elements: self.elements.iter().map(|x| x.clone_ref(py)).collect(),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl List {
    #[new]
    fn new(elements: Vec<PyObject>, span: Span) -> Self {
        List { elements, span }
    }
    fn __repr__(&self) -> String {
        format!("List({:?})", self.elements)
    }
}
// MARK: Map
#[pyclass]
pub struct Map {
    #[pyo3(get)]
    recursive: bool,
    #[pyo3(get)]
    bindings: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                recursive: self.recursive,
                bindings: self.bindings.iter().map(|x| x.clone_ref(py)).collect(),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl Map {
    #[new]
    fn new(recursive: bool, bindings: Vec<PyObject>, span: Span) -> Self {
        Map { recursive, bindings, span }
    }
    fn __repr__(&self) -> String {
        format!("Map(recursive={}, {:?})", self.recursive, self.bindings)
    }
}

// ==================== PATH & URI ====================
// MARK: Path
#[pyclass]
pub struct Path {
    #[pyo3(get)]
    parts: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                parts: self.parts.iter().map(|x| x.clone_ref(py)).collect(),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl Path {
    #[new]
    fn new(parts: Vec<PyObject>, span: Span) -> Self {
        Path { parts, span }
    }
    fn __repr__(&self) -> String {
        format!("Path({:?})", self.parts)
    }
}
// MARK: Uri
#[pyclass]
#[derive(Clone)]
pub struct Uri {
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
// MARK: PropertieAccess
#[pyclass]
pub struct PropertyAccess {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    attribute_path: Vec<PyObject>,
    #[pyo3(get)]
    default: Option<PyObject>,
    #[pyo3(get)]
    span: Span,
}

impl Clone for PropertyAccess {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                expression: self.expression.clone_ref(py),
                attribute_path: self.attribute_path.iter().map(|x| x.clone_ref(py)).collect(),
                default: self.default.as_ref().map(|x| x.clone_ref(py)),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl PropertyAccess {
    #[new]
    fn new(expression: PyObject, attribute_path: Vec<PyObject>, default: Option<PyObject>, span: Span) -> Self {
        PropertyAccess { expression, attribute_path, default, span }
    }
    fn __repr__(&self) -> String {
        format!("PropertyAccess({:?}, {:?}, {:?})", self.expression, self.attribute_path, self.default)
    }
}

// ==================== REMAINING TYPES ====================
// MARK: SearchNixPath
#[pyclass]
#[derive(Clone)]
pub struct SearchNixPath {
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

// MARK: NixString
#[pyclass]
pub struct NixString {
    #[pyo3(get)]
    parts: Vec<PyObject>,
    #[pyo3(get)]
    span: Span,
}

impl Clone for NixString {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                parts: self.parts.iter().map(|x| x.clone_ref(py)).collect(),
                span: self.span.clone(),
            }
        })
    }
}

#[pymethods]
impl NixString {
    #[new]
    fn new(parts: Vec<PyObject>, span: Span) -> Self {
        NixString { parts, span }
    }
    fn __repr__(&self) -> String {
        format!("String({:?})", self.parts)
    }
}
// MARK: UnaryOperation
#[pyclass]
pub struct UnaryOperation {
    #[pyo3(get)]
    operator: PyObject,
    #[pyo3(get)]
    operand: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for UnaryOperation {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                operator: self.operator.clone_ref(py),
                operand: self.operand.clone_ref(py),
                span: self.span.clone(),
            }
        })
    }
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
// MARK: With
#[pyclass]
pub struct With {
    #[pyo3(get)]
    expression: PyObject,
    #[pyo3(get)]
    target: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for With {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                expression: self.expression.clone_ref(py),
                target: self.target.clone_ref(py),
                span: self.span.clone(),
            }
        })
    }
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
// MARK: BindingInherit
#[pyclass]
pub struct BindingInherit {
    #[pyo3(get)]
    from_: Option<PyObject>,
    #[pyo3(get)]
    attributes: PyObject,
    #[pyo3(get)]
    span: Span,
}

impl Clone for BindingInherit {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                from_: self.from_.as_ref().map(|x| x.clone_ref(py)),
                attributes: self.attributes.clone_ref(py),
                span: self.span.clone(),
            }
        })
    }
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
// MARK: BindingKeyValue
#[pyclass]
pub struct BindingKeyValue {
    #[pyo3(get)]
    from_: PyObject,
    #[pyo3(get)]
    to: PyObject,
}

impl Clone for BindingKeyValue {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            Self {
                from_: self.from_.clone_ref(py),
                to: self.to.clone_ref(py),
            }
        })
    }
}

#[pymethods]
impl BindingKeyValue {
    #[new]
    fn new(from_: PyObject, to: PyObject) -> Self { Self { from_, to } }
    fn __repr__(&self) -> String { format!("KeyValue({:?})", self.from_) }
}