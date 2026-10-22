# Operator Function

`func` statement followed by `infix`, `prefix` and `postfix` is a operator function statement which defines the implementation of operator for the specific operand types.

## Specifications

Operator functions must have a return type. Multiple operator functions can be defined for the same symbol with different operand types, allowing operator overloading.

The parameter name does not have to be `lhs`, `rhs` or `operand`, so you can set any parameter name.

## Syntax

```
func infix <Symbol>(lhs: <Type>, rhs: <Type>) -> <Type> { }

func prefix <Symbol>(operand: <Type>) -> <Type> { }

func postfix <Symbol>(operand: <Type>) -> <Type> { }
```

## Example 1 --- Infix Operator

This example program defines a custom infix operator `%%` that calculates the average of two operands.

```kasl
import std

operator infix %% {
    precedence: 50,
    associativity: left
}

func infix %%(lhs: Float, rhs: Float) -> Float {
    return (lhs + rhs) / 2.0
}

func infix %%(lhs: Int, rhs: Int) -> Int {
    return (lhs + rhs) / 2
}

func main() {
    let float_avg = 3.0 %% 7.0 // 5.0
    let int_avg = 3 %% 7 // 5
}
```

## Example 2 --- Prefix Operator

The following program defines a custom prefix operator `--` that calculates a decremented value.

```kasl
import std

operator prefix -- {
    precedence: 110
}

func prefix --(operand: Int) -> Int {
    return operand - 1
}

func main() {
    let x = --5
}
```

## Example 3 --- Postfix Operator

This example program defines a custom postfix operator `%` that converts percentage to ratio.

```kasl
import std

operator postfix % {
    precedence: 110
}

func postfix %(operand: Float) -> Float {
    return operand / 100.0
}

func main() {
    let ratio = 75.0%
}
```
