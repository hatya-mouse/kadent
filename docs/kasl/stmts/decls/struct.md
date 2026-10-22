# Struct

`struct` statement declares a new struct.

## Specifications

A struct is a custom data type that groups related values together as fields. Each field is declared with [`var`](./field_var.md) and must have a default value.

To create a new instance of a struct, call it like a function with no arguments, such as `Foo()`. This creates a new instance with all fields set to their default values.

Fields and instance functions can be accessed with `.`, such as `instance.field` or `instance.method()`.

Static functions belong to the type itself rather than instance, and can be called as `Foo.method()`.

If you would like to create struct instances with non-default values, define the functions using `static` just like `Foo.new()`.

For more information about struct fields and member functions, visit the [struct field](field_var.md) page and [function](./func.md) page.

## Syntax

```
struct <Struct Name> {
    // Struct Field
    var <Field> = <Default Value>
    
    // Instance Function
    func <Method>() { }
    
    // Static Function
    static func <Method>() { }
}
```

## Example 1 --- Basic

This example program declares a struct called `Point` and initializes it using `Point()`.

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

## Example 2 --- Member Functions

The program below creates a struct called `Circle` which has a static function to initialize a new circle with the given radius and a instance function that scales the circle by the given factor.

```kasl
import std

struct Circle {
    var radius = 0.0

    static func new(radius r: Float) -> Circle {
        var c = Circle()
        c.radius = r
        return c
    }

    func scale(factor: Float) {
        self.radius = self.radius * factor
    }
}

func main() {
    var c = Circle.new(radius: 5.0)
    c.scale(2.0)
}
```
