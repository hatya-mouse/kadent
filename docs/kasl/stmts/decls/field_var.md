# Struct Field

`var` statement in struct declaration is used to declare a field of a struct.

## Specifications

Like other variable and constant declarations, it is possible to omit the type name, or you can specify the type of the variable.
If the value type is not specified, it will be inferred from the default value.

## Syntax

```
struct <Struct Name> {
    // Without type name
    var <Name> = <Default Value>

    // With type name
    var <Name>: <Type> = <Default Value>
}
```

## Example

This example program declares a struct called `Point` with fields called `x` and `y`.

```kasl
struct Point {
    var x = 0.0
    var y = 0.0
}

func main() {
    var p = Point()
    p.x = 3.0
    p.y = 4.0
}
```
