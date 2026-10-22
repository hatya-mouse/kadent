# Var

`var` statement declares a new local variable, which is accessible only inside the scope.

## Specifications

The variable is a value you can change multiple times.

Like other variable declarations, it is possible to omit the type name, or you can specify the type of the variable.
If the value type is not specified, it will be inferred from the default value.

## Syntax

```
// Without type name
var <Name> = <Default Value>

// With type name
var <Name>: <Type> = <Default Value>
```

## Example

This example program declares a variable called `i` for loop count tracking.

```kasl
import std

func main() {
    var i = 0
    loop 10 {
        do_something(i)
        i = i + 1
    }
}

func do_something(i: Int) {
    // Do something
}
```
