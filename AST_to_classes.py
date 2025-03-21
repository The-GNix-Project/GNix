""" 
To oxidize once I figure out the logic
"""
from typing import Optional, Union
from typing import List as PyList

class Addition: value = "Addition"
class Concatenation: value = "Concatenation"
class EqualTo: value = "EqualTo"
class GreaterThan: value = "GreaterThan"
class GreaterThanOrEqualTo: value = "GreaterThanOrEqualTo"
class Division: value = "Division"
class Implication: value = "Implication"
class LessThan: value = "LessThan"
class LessThanOrEqualTo: value = "LessThanOrEqualTo"
class LogicalAnd: value = "LogicalAnd"
class LogicalOr: value = "LogicalOr"
class Multiplication: value = "Multiplication"
class NotEqualTo: value = "NotEqualTo"
class Subtraction: value = "Subtraction"
class Update: value = "Update"

BinaryOperator = Union[
    Addition,
    Concatenation,
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    Division,
    Implication,
    LessThan,
    LessThanOrEqualTo,
    LogicalAnd,
    LogicalOr,
    Multiplication,
    NotEqualTo,
    Subtraction,
    Update,
]

class Not: value = "Not"
class Negate: value = "Negate"

UnaryOperator = Union[
    Not,
    Negate
]

Operator = Union[UnaryOperator, BinaryOperator]

class Position:
    line: int
    column: int

class Span:
    start: Position
    end: Position

class Identifier:
    id: str
    span: Span

class Assert:
    expression: "Expression"
    target: "Expression"
    span: Span

class BinaryOperation:
    left: "Expression"
    operator: BinaryOperator
    right: "Expression"

class Error:
    message: str
    span: Span 

class Float:
    value: str
    span: Span

class FunctionHeadSimple:
    identifier: Identifier
    span: Span

class FunctionHeadDestructuredArgument:
    identifier: str
    default: Optional["Expression"]

class FunctionHeadDestructured:
    ellipsis: bool
    identifier: Identifier
    arguments: FunctionHeadDestructuredArgument

FunctionHead = Union[
    FunctionHeadSimple,
    FunctionHeadDestructured
]

class Function:
    head: "FunctionHead"
    body: "Expression"
    span: Span

class FunctionApplication:
    function: Function
    arguments: "Expression" 

class PartInterpolation:
    expression: "Expression"

class PartRaw:
    content: str
    span: Span

Part = Union[ 
    "Expression",
    PartInterpolation,
    PartRaw
]

class HasAttribute:
    expression: "Expression"
    attribute_path: PyList[Part]

class IfThenElse:
    predicate: "Expression"
    then: "Expression"
    else_: "Expression"
    span: Span

class IndentedString:
    parts: PyList[Part]
    span: Span  

class Integer:
    value: str
    span: Span

class BindingKeyValue:
    from_: Part
    to: "Expression"

class BindingInherit:
    from_: Optional["Expression"]
    attributes: Part
    span: Span

Binding = Union[ 
    BindingInherit,
    BindingKeyValue
]

class LetIn:
    bindings: PyList[Binding]
    target: "Expression"
    span: Span  

class List:
    elements: PyList["Expression"]
    span: Span

class Map:
    recursive: bool
    bindings: PyList[Binding]
    span: Span  

class Path:
    parts: PyList[Part]
    span: Span

class PropertyAccess:
    expression: "Expression"
    attribute_path: PyList[Part]
    default: Optional["Expression"]

class SearchNixPath:
    path: str
    span: Span

class String:
    parts = PyList[Part]
    span = Span

class UnaryOperation:
    operator: UnaryOperator
    operand: "Expression"
    span: Span

class Uri:
    uri: str
    span: Span  

class With:
    expression: "Expression"
    target: "Expression"
    span: Span

Expression = Union[
    Assert,
    BinaryOperation,
    Error,
    Float,
    Function,
    FunctionApplication,
    HasAttribute,
    Identifier,
    IfThenElse,
    IndentedString,
    Integer,
    LetIn,
    List,
    Map,
    Path,
    PropertyAccess,
    SearchNixPath,
    String,
    UnaryOperation,
    Uri,
    With,
]
