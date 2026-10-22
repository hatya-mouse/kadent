# State

`state` statement declares a new state variable.

## Specifications

The state variable preserves the containing value over iterations, which is designed to be used for delay, for example.

Like other variable and constant declarations, it is possible to omit the type name, or you can specify the type of the variable.
If the value type is not specified, it will be inferred from the default value.

## Syntax

```
// Without type name
state <Name> = <Default Value>

// With type name
state <Name>: <Type> = <Default Value>
```

## Example

This example program declares a new state variable called `counter` and increments it in the every iteration.

```kasl
import std

state counter = 0

func main() {
    counter = counter + 1
}
```
