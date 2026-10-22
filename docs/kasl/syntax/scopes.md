# Scopes

Scopes are the regions of a program where certain variables and functions are accessible.

## Global Scope

In KASL, global scope means the top-level scope of the program, which is not enclosed in brackets.

```kasl
// This is the global scope

struct Foo {
    // This is not the global scope
}

func main() {
    // This is not the global scope
}
```

Variables and functions declared in the global scope are accsessible anywhere in the program.
They are visible to the external program when the file is being imported.

## Local Scope

Local scope is the region of a program that is enclosed in the curly brackets of a function, or curly brackets inside the function body.

```kasl
func main() {
    // This is the scope of main function
    
    {
        // This is the scope of the inner block
    }
}
```

## Accessibility

Variables declared in a local scope are only accessible within that scope and its nested scopes. They are not visible outside of that scope, whereas variables and functions declared in the global scope are accessible anywhere in the program.

```kasl
// Declare a global constant
let pi = 3.14

func main() {
    // Declare a local variable
    let radius = 5.0
    
    {
        // This variable is only visible in this block
        let diameter = radius * 2
    }
    
    // This causes an error because the variable diameter is not accessible here!
    // let circumference = diameter * pi
    
    // Calculate the area of a circle
    let area = pi * radius * radius
}
```

## Multiple Files

When another KASL program is imported, only the variables and functions declared in the global scope of the imported file are accessible. Variables declared in local scopes of the imported file are not accessible.

For example, we have `module.kasl` and `main.kasl` as follows:

`module.kasl`
```kasl
// This variable is in the global scope and can be accessed from main.kasl
let pi = 3.14

func calculate_area(radius: Float) -> Float {
    // This function is in the global scope and can be accessed from main.kasl
    return pi * radius * radius
}
```

`main.kasl`
```kasl
import module

func main() {
    let radius = 5.0
    
    // Access the global variable from module.kasl
    let area = module.pi * radius * radius
    
    // Call the function from module.kasl
    let area2 = module.calculate_area(radius)
}
```

In this case, `pi` and `calculate_area` from `module.kasl` are accessible in `main.kasl` because they are declared in the global scope of `module.kasl`.
