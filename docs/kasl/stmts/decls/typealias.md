# Typealias

`typealias` statement creates an alias for any type. This is useful for giving a more descriptive name to a complex type, especially arrays.

## Syntax

```
typealias <Alias Name> = <Actual Type>
```

## Example

This example program creates an alias called `Vector2`.

```kasl
import std

typealias Vector2 = [Float; 2]

input in_vector = [0.0; 2]
output doubled = [0.0; 2]

func main() {
    doubled = double(in_vector)
}

func double(vector: Vector2) -> Vector2 {
    return [vector[0] * 2, vector[1] * 2]
}
```
