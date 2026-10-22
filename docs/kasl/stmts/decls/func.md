# Function

`func` statement declares a function in the global scope or struct declarations.

## Specifications

Functions can have multiple parameters and single return value. If no return type is specified, the function is expected to return no value.

Every parameter can have an argument label for clarity. If the argument label is specified on function call, the argument will match to be the first parameter that has the same argument label. The subsequent arguments will match to the parameters following the labeled one, so the the order of the parameters cannot be changed. Any parameters skipped in this process will use their default values.

In instance functions (functions in struct statement which are not marked `static`), `self` keyword can be used to specify the instance where the function is called.

Static functions can be called on struct name, enabling to create a function which is not called on an instance but belongs to the struct.

## Syntax

```
// Basic
func <Name>() { }

// With parameters
func <Name>(<Param>: <Type>) { }

// With return value
func <Name>(<Param>: <Type>) -> <Type> { }

// With argument label
func <Name>(<Label> <Param>: <Type>) { }

// With default value
func <Name>(<Param> = <Default Value>) { }

// Static function
static func <Name>() { }
```

## Example 1 --- Basic

The program below declares a function called greet, which takes no parameters and returns nothing.

```kasl
func greet() {
    // Do something
}

func main() {
    greet()
}
```

## Example 2 --- Parameters and Return Value

The program below declares some functions with parameters and return types, and they can be called as shown in the `main` function.

```kasl
import std

func add(lhs: Int, rhs: Int) -> Int {
    return lhs + rhs
}

func multiply(times a: Int, by b: Int) -> Int {
    return a * b
}

func increment(value: Int = 0) -> Int {
    return value + 1
}

// Use the declared functions!
func main() {
    add(1, 2) // 3
    multiply(times: 3, by: 4) // 12
    increment() // 1
    increment(10) // 11
}
```

## Example 3 --- Instance Function

The following program creates an instance function called `increment` and `add` which mutate the struct field `count`.

```kasl
struct Counter {
    var count = 0

    func increment() {
        self.count = self.count + 1
    }

    func add(amount: Int) {
        self.count = self.count + amount
    }
}

func main() {
    var c = Counter()
    c.increment() // 1
    c.add(5) // 6
}
```

## Example 4 --- Static Function

The program below creates a static function called `unit`, which can be called by `Circle.unit()` using its belonging type name `Circle`.

```kasl
struct Circle {
    var radius = 0.0

    static func unit() -> Circle {
        var c = Circle()
        c.radius = 1.0
        return c
    }
}

func main() {
    let unit_circle = Circle.unit()
}
```
