# Block

A block is a sequence of statements enclosed in `{}`.

## Specifications

A block creates a new scope. Variables and constants declared inside a block are not accessible outside of it.

## Syntax

```
{
    <Statements>
}
```

## Example

This example program declares two variables named `x` and `y`, but `y` is not accessible outside of the inner scope.

```kasl
func main() {
    let x = 1
    {
        let y = 2
    }
    // y is not accessible here.
}
```
