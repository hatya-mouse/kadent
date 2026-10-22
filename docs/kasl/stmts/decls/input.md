# Input

`input` statement declares a new input.

## Specifications

Input variable will be given from the execution host, and it is not able to assign a value to the input after declaration.

Like other variable and constant declarations, it is possible to omit the type name, or you can specify the type of the variable.
If the value type is not specified, it will be inferred from the default value.

## Syntax

```
// Without type name
input <Name> = <Default Value>

// With type name
input <Name>: <Type> = <Default Value>
```

## Example

This example program declares an input and uses it to calculate a value.

```kasl
import std

input in = 0

func main() {
    let doubled = in * 2
}
```
