# Declaration Statements

Declaration statements are the statements that can be used in the global scopes.

## Global Statements

These are the statements can be used in the global scope to declare symbols such as variables, structs and functions.

| Keyword | Description | Link |
| --- | --- | --- |
| import | Imports another kasl program. | [Import](import.md) |
| typealias | Names the specific type. | [Typealias](typealias.md) |
| func | Defines a function. | [Function](func.md) |
| input | Defines an input. | [Input](input.md) |
| output | Defines an output variable. | [Output](output.md) |
| state | Defines a state variable. | [State](state.md) |
| let | Defines a global constant. | [Let](global_let.md) |
| struct | Defines a struct. | [Struct](struct.md) |
| operator infix / prefix / postfix | Defines the precedence and the associativity for the operator. | [Operator Definition](operator.md) |
| func infix / prefix / postfix | Declares the implementation of an operator for the specific operand types. | [Operator Function](op_func.md) |

## Struct Statements

Below are the statements can be used inside struct declarations.

| Keyword | Description | Link |
| --- | --- | --- |
| var | Defines a struct field. | [Struct Field](field_var.md) |
| func | Defines a member function for the struct. | [Function](func.md) |
