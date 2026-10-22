# Let

`let` statement declares a new local constant, which is accessible only inside the scope.

## Specifications

The constant is a value you cannot change once it is declared.

Like other variable declarations, it is possible to omit the type name, or you can specify the type of the variable.
If the value type is not specified, it will be inferred from the default value.

## Syntax

```
// Without type name
let <Name> = <Default Value>

// With type name
let <Name>: <Type> = <Default Value>
```

## Example

This example program uses constants to break down the calculation.

```kasl
import std

func main() {
    let x = 10
    let doubled = x * 2
    let result = doubled + 5 // 25
}
```
