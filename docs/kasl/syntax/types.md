# Types

Every value in KASL has a type. The type of a value determines what operations can be performed on it and how much memory it occupies.
KASL has three primitive types, arrays, structs, and typealiases.

## Primitive Types

KASL has three primitive types: `Int`, `Float`, and `Bool`.
`Int` is a 32-bit signed integer, `Float` is a 32-bit floating-point number, and `Bool` is a boolean value that can be either `true` or `false`.

```kasl
let my_int: Int = 42
let my_float: Float = 3.14
let my_bool: Bool = true
```

## Arrays

KASL supports fixed-size arrays of any type. The size of an array is determined at compile time and cannot be changed at runtime. Arrays are indexed starting from 0.
The type of an array is denoted by the type of its elements and its size, written as `[T; N]`, where `T` is the type of the elements and `N` is the size of the array.
For example, an array of 5 integers can be declared as follows:

```kasl
let my_array: [Int; 5] = [1, 2, 3, 4, 5]
```

This program declares an array of 5 integers and initializes it with the values 1, 2, 3, 4 and 5.

If you initialize an array with the same value for all elements, you can use the following syntax:

```kasl
let my_array: [Int; 5] = [0; 5]
```

KASL supports multi-dimensional arrays as well. For example, a 2D array of floats with 3 rows and 4 columns can be declared as follows:

```kasl
let my_2d_array: [[Float; 4]; 3] = [
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0]
]
```

## Structs

Structs are user-defined types that can contain multiple fields of different types.
Let's say we want to define a struct to represent a point in 2D space. We can define it as follows:

```kasl
struct Point {
    var x = 0.0
    var y = 0.0
}
```

This defines a struct named `Point` with two fields: `x` and `y`, both of type `Float`.
`0.0` is the default value for both fields. When you create a new instance of `Point`, the fields will be initialized to these default values.
We can create an instance of this struct and access its fields like this:

```kasl
func main() {
    let p = Point()
    var x = p.x // x is 0.0
    p.x = 5.0 // p.x is set to 5.0
    x = p.x // x is now 5.0
}
```

You cannot define a cyclic struct, meaning a struct cannot contain a field that is of the same type as the struct itself. This is to prevent infinite recursion when trying to create an instance of the struct.
For example, the following code will cause a compilation error:

```kasl
struct Node {
    var value = 0
    var node = Node() // This will cause a compilation error because it creates a cyclic struct
}
```

This also happens if the cycle is indirect, for example:

```kasl
struct A {
    var b = B()
}

struct B {
    var c = C()
}

struct C {
    var a = A()
}
```

## Typealiases

Typealiases allow you to create a new name for an existing type. This can be useful for improving code readability or for creating more descriptive names for complex types.

```kasl
typealias Sample = [Float; 2]
```

In this example, we create a typealias named `Sample` that is an alias for the `[Float; 2]` type. We can now use `Sample` in our code to refer to `[Float; 2]`, which can make our code more readable when we are specifically referring to samples.

```kasl
output out = [0.0; 2]

func main() {
    let my_sample: Sample = [0.5, 0.8]
    out = my_sample
}
```

Typealias just creates a new name for an existing type, it does not create a new type. This means that `Sample` and `[Float; 2]` are interchangeable in the code.
You can set `[Float; 2]` values to `Sample` variables and vice versa without any issues.
