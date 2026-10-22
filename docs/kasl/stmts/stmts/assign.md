# Assign

Assign statement assigns a value to a specific variable.

## Specifications

`input` and `let` declarations cannot be assigned after declaration.

## Syntax

```
<Variable> = <Value>

// Struct field
<Instance>.<Field> = <Value>
```

## Example

This example program assigns a value to a variable and struct fields.

```kasl
struct Point {
    var x = 0.0
    var y = 0.0
}

func main() {
    var n = 0
    n = 5

    var p = Point()
    p.x = 3.0
    p.y = 4.0
}
```
