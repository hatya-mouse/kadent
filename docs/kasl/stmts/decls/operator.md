# Operator Definition

`operator` statement defines a custom operator and its precedence and associativity.
The implementation of the operator is defined separately using the [operator function](./op_func.md) statement.

## Specifications

`infix` operators take two operands on both sides, `prefix` operators take a single operand on the right, and `postfix` operators take a single operand on the left.

Precedence determines the order of evaluation when multiple operators appear in an expression. A higher precedence value means the operator binds more tightly.

Associativity determines how operators of the same precedence are grouped.
`left` groups from the left, `right` groups from the right, and `none` disallows chaining operators of the same precedence.
Prefix and postfix operators do not have associativity.

## Syntax

```
operator infix <Symbol> {
    precedence: <Precedence>,
    associativity: <Associativity>
}

operator prefix <Symbol> {
    precedence: <Precedence>
}

operator postfix <Symbol> {
    precedence: <Precedence>
}
```

## Example

This example program defines a custom operator `**` and declares its implementation for `Float` using [operator function](./op_func.md) statement.

```kasl
import std

operator infix ** {
    precedence: 95,
    associativity: right
}

func infix **(lhs: Float, rhs: Float) -> Float {
    return std.float.pow(lhs, rhs)
}

func main() {
    let square = 5.0 ** 2.0
}
```
