pub mod parser;
pub mod utils;

use pyo3::prelude::*;
use pyo3::Bound;
use pyo3::wrap_pyfunction;

use parser::grammar::*;
use utils::*;

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