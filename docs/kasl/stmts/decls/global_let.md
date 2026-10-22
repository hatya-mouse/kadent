# Let

`let` statement declares a new constant.

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

This example program declares a constant called `pi`, and calculates sine value for pi.

```kasl
import std

let pi = 3.1415926535

func main() {
    let value = std.float.sin(pi) // 0.0
}
```
