# Return

`return` statement exits from the function and returns a value from the function.

## Specifications

The return statement must not return a value if the function does not have a return type, and must return a value if the function has a return type specified.

In the function with a return type specified, all execution paths must have return statement that returns a value.
For example, the following program will cause an error:

```kasl
import std

func main() {
    do_something(5)
}

func do_something(number: Int) -> Int {
    if number > 0 {
        return number
    }
}
```

This causes an error because the do_something function will not return a value if number equals zero or is under zero.

## Syntax

```
// No return value
return

// With return value
return <Value>
```

## Example 1 --- No Return Value

The following program conditionally returns and exits from function.
This does not cause an error because the `main` function does not have return type specified.

```kasl
import std

func main() {
    var x = 10
    if x > 100 {
        return
    }
    x = x + 1
}
```

## Example 2 --- With Return Value

The following program declares a function called add that returns an added value.

```kasl
import std

func main() {
    let result = add(3, 5)
}

func add(lhs: Int, rhs: Int) -> Int {
    return lhs + rhs
}
```
