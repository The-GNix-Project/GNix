## Definitions
### Core Structures
- ` Position(line: i64, column: i64)`
- `Span(start: Position, end: Position)`
- `Identifier(id: String, span: Span)`
- `Error(message: String, span: Span)`
- `Float(value: String, span: Span)`
- `Integer(value: String, span: Span)`

---

### Operators
- `Addition()`
- `Concatenation()`
- `EqualTo()`
- `GreaterThan()`
- `GreaterThanOrEqualTo()`
- `Division()`
- `Implication()`
- `LessThan()`
- `LessThanOrEqualTo()`
- `LogicalAnd()`
- `LogicalOr()`
- `Multiplication()`
- `NotEqualTo()`
- `Subtraction()`
- `Update()`
- `Not()`
- `Negate()`

---

### Function Structures
- `FunctionHeadDestructuredArgument(identifier: String, default: Option<PyObject>)`
- `FunctionHeadDestructured(ellipsis: bool, identifier: Identifier, arguments: FunctionHeadDestructuredArgument, span: Span)`
- `FunctionHeadSimple(identifier: Identifier, span: Span)`
- `Function(head: PyObject, body: PyObject, span: Span)`
- `FunctionApplication(function: PyObject, arguments: PyObject, span: Span)`

---

### Parts
- `PartInterpolation(expression: PyObject, span: Span)`
- `PartRaw(content: String, span: Span)`

---

### Expressions
- `BinaryOperation(left: PyObject, operator: PyObject, right: PyObject, span: Span)`
- `Assert(expression: PyObject, target: PyObject, span: Span)`
- `HasAttribute(expression: PyObject, attribute_path: Vec<PyObject>, span: Span)`
- `IndentedString(parts: Vec<PyObject>, span: Span)`
- `IfThenElse(predicate: PyObject, then: PyObject, else_: PyObject, span: Span)`
- `LetIn(bindings: Vec<PyObject>, target: PyObject, span: Span)`
- `List(elements: Vec<PyObject>, span: Span)`
- `Map(recursive: bool, bindings: Vec<PyObject>, span: Span)`
- `Path(parts: Vec<PyObject>, span: Span)`
- `Uri(uri: String, span: Span)`
- `PropertyAccess(expression: PyObject, attribute_path: Vec<PyObject>, default: Option<PyObject>, span: Span)`
- `SearchNixPath(path: String, span: Span)`
- `NixString(parts: Vec<PyObject>, span: Span)`
- `UnaryOperation(operator: PyObject, operand: PyObject, span: Span)`
- `With(expression: PyObject, target: PyObject, span: Span)`

---

### Bindings
- `BindingInherit(from_: Option<PyObject>, attributes: PyObject, span: Span)`
- `BindingKeyValue(from_: PyObject, to: PyObject)`

## List
Position<br>
Span<br>
Identifier<br>
Error<br>
Float<br>
Integer<br>
Addition<br>
Concatenation<br>
EqualTo<br>
GreaterThan<br>
GreaterThanOrEqualTo<br>
Division<br>
Implication<br>
LessThan<br>
LessThanOrEqualTo<br>
LogicalAnd<br>
LogicalOr<br>
Multiplication<br>
NotEqualTo<br>
Subtraction<br>
Update<br>
Not<br>
Negate<br>
FunctionHeadDestructuredArgument<br>
FunctionHeadDestructured<br>
FunctionHeadSimple<br>
Function<br>
FunctionApplication<br>
PartInterpolation<br>
PartRaw<br>
BinaryOperation<br>
Assert<br>
HasAttribute<br>
IndentedString<br>
IfThenElse<br>
LetIn<br>
List<br>
Map<br>
Path<br>
Uri<br>
PropertyAccess<br>
SearchNixPath<br>
NixString<br>
UnaryOperation<br>
With<br>
BindingInherit<br>
BindingKeyValue<br>