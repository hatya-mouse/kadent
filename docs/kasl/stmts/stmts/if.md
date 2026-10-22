# If

If statement conditionally executes a sequence of statements.

## Specifications

The condition does not need parentheses and has to be of `Bool` type.

## Syntax

```
if <Condition> {
    <Statements>
}

if <Condition> {
    <Statements>
} else {
    <Statements>
}

if <Condition> {
    <Statements>
} else if <Condition> {
    <Statements>
} else {
    <Statements>
}
```

## Example

The following example conditionally executes assign statement.

```kasl
import std

func main() {
    var x = 10
    if x > 5 {
        x = 0
    }
    
    var y = 3
    if y > 5 {
        y = 1
    } else {
        y = 0
    }
}
```
