# Statements

This documentation describes the statements that can be used in KASL.

## Declaration Statements

Declaration statements are the statements that can be used in the global scopes.

### Global Statements

These are the statements which can be used in the global scope to declare symbols such as variables, structs and functions.

| Keyword | Description | Link |
| --- | --- | --- |
| import | Imports another kasl program. | [Import](./decls/import.md) |
| typealias | Names the specific type. | [Typealias](./decls/typealias.md) |
| func | Defines a function. | [Function](./decls/func.md) |
| input | Defines an input. | [Input](./decls/input.md) |
| output | Defines an output variable. | [Output](./decls/output.md) |
| state | Defines a state variable. | [State](./decls/state.md) |
| let | Defines a global constant. | [Let](./decls/global_let.md) |
| struct | Defines a struct. | [Struct](./decls/struct.md) |
| operator infix / prefix / postfix | Defines the precedence and the associativity for the operator. | [Operator Definition](./decls/operator.md) |
| func infix / prefix / postfix | Declares the implementation of an operator for the specific operand types. | [Operator Function](./decls/op_func.md) |

### Struct Statements

Below are the statements can be used inside struct declarations.

| Keyword | Description | Link |
| --- | --- | --- |
| var | Defines a struct field. | [Struct Field](./decls/field_var.md) |
| func | Defines a member function for the struct. | [Function](./decls/func.md) |

## Statements

Statements can be used inside functions, and creates the actual execution logic.

| Statement | Description | Link |
| --- | --- | --- |
| {} | Creates a scope. | [Block](./stmts/block.md) |
| var | Declares a local variable. | [Var](./stmts/var.md) |
| let | Declares a local constant. | [Let](./stmts/let.md) |
| = | Assigns a value to the variable or its field. | [Assign](./stmts/assign.md) |
| if / if-else | Conditionally execute the statements. | [If](./stmts/if.md) |
| return | Exits from the function with a return value. | [Return](./stmts/return.md) |
| loop | Creates a loop that will be executed specific times. | [Loop](./stmts/loop.md) |
