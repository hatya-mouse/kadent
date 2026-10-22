# Loop

`loop` statement executes a sequence of statements a specific number of times.

## Specifications

The number of iterations must be constant, not a variable.

If you need an index inside the loop, declare a variable and manually increment it.

## Syntax

```
loop <Count> {
    <Statements>
}
```

## Example

The following program calculates the sum of the variable `i` for each iterations.

```kasl
import std

func main() {
    var sum = 0
    var i = 0
    loop 10 {
        sum = sum + i
        i = i + 1
    }
}
```
