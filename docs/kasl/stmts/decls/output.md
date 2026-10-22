# Output

`output` statement declares a new output variable.

## Specifications

The value in the output variables will be sent back to the execution host as an output of the program.
If no value is assigned to the output variable after declaration, the default value will be sent to the host.

Like other variable and constant declarations, it is possible to omit the type name, or you can specify the type of the variable.
If the value type is not specified, it will be inferred from the default value.

## Syntax

```
// Without type name
output <Name> = <Default Value>

// With type name
output <Name>: <Type> = <Default Value>
```

## Example

This example program declares a new output variable called `out` and assigns an integer value `5` to the variable.

```kasl
output out = 0

func main() {
    out = 5
}
```
