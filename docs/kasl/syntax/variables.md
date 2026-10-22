# Variables

KASL supports variables, which are used to store values that can be used throughout a program. Variables can be assigned values using the `=` operator, and their values can be accessed by simply using their names.
Every variable in KASL has a type, which determines the kind of data it can hold. The type of a variable is determined at the time of its declaration and cannot be changed later. KASL supports several basic types, including `Int`, `Float`, and `Bool` which are called "Primitive Types". See more about types in the [Types](./types.md) section.

KASL has 7 types of variables, which we will cover in the following sections — but don't worry, they are all pretty similar and should be easy for you to understand!

## Index

| Item | Description |
| --- | --- |
| [Local Variables](#1-local-variables) | Stores the value in functions. |
| [Local Constants](#2-local-constants) | Declares an unchangeable value. |
| [Global Input Variables](#3-global-input-variables) | Receives the input data from the host. |
| [Global Output Variables](#4-global-output-variables) | Passes the output data to the host. |
| [Global State Variables](#5-global-state-variables) | Used to preserve data over execution iterations. |
| [Global Constants](#6-global-constants) | Declares an unchangeable value which can be used across the program. |
| [Struct Fields](#7-struct-fields) | Stores the value in structs. |

## 1. Local Variables

Local variables are the variables that can be declared and used within a function. They are only accessible within the function they are declared in and cannot be accessed outside of it.

The most basic way to declare a local variable is to simply specify its type and name, and the initial value for the variable. For example:

```kasl
func main() {
    var x: Int = 10
}
```

In KASL, you can omit the type of a variable, because the compiler automatically infers the type based on the initial value. So the above code can be rewritten as:

```kasl
func main() {
    var x = 10
}
```

This is much simpler and more concise. We'll be using this style of variable declaration for the variables after this point.

## 2. Local Constants

Local constants are similar to local variables, but their values cannot be changed once they are assigned. They are declared using the `let` keyword instead of `var`. For example:

```kasl
func main() {
    let x = 10
    // This will cause an error because x is a constant:
    // x = 20
}
```

## 3. Global Input Variables

Input variables are special variables that are used to receive input from the execution host. Execution host can be the DAW, CLI, or any other environment that supports KASL. Input variables are declared using the `input` keyword.
Input variables cannot be assigned values within the program after they are declared.
For example:

```kasl
input tempo = 0.0
```

You need to specify the initial value for an input variable for safety reasons, but it'll be overwritten with the actual input value from the execution host when the program is run.

## 4. Global Output Variables

Just like input variables, output variables are special variables that are used to send output to the execution host. They are declared using the `output` keyword. For example:

```kasl
output audio = 0.0
```

Output variables can be assigned values within the program, and the final result will be sent to the execution host when the program is run.

## 5. Global State Variables

State variables are special variables that are used to store state information that can be accessed across different iterations.
Because KASL is designed for real-time audio processing, the main function will be called for every single samples, and state variables are used to preserve the value over main function calls.
This is the recommended way to implement things like delay effects, where you need to store the previous samples to calculate the output for the current sample.
They are declared using the `state` keyword. For example:

```kasl
state delay_buffer = [0.0; 44100]
```

## 6. Global Constants

Global constants are similar to local constants, but they are accessible throughout the entire program. They are declared using the `let` keyword. For example:

```kasl
let pi = 3.14

func main() {
    // You can access pi here
}
```

## 7. Struct Fields

Unlike the previous 6 types of variables, struct fields are the variables that are used to store data within a struct. They are declared within the struct definition using the `var` keyword, and can be accessed using the dot notation. For example:

```kasl
struct Point {
    var x = 0.0
    var y = 0.0
}
```

When you initialize a struct, the default values for the fields will be used. For example:

```kasl
func main() {
    let p = Point()
    // p.x is 0.0
    // p.y is 0.0
}
```
