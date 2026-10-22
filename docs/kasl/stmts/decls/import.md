# Import

`import` statement imports an external KASL file in search paths and makes it accessible inside the program.

## Specifications

Symbols in the imported file can be accessed by the file name without the extension `.kasl`, just like `foo.pi` in the example below.
If you would like to import a file in the subdirectory of the directory at the search path, you can write the path using `/` (slash) to access its subdirectories.

## Syntax

```
import <Path to the File>
```

## Example 1

Here's a example of a KASL program, which imports `foo.kasl`, calls its function and accesses its constant.

`foo.kasl`
```kasl
let pi = 3.1415926535

func do_something(value: Float) {
    // Do something
}
```

`main.kasl`
```kasl
import foo

func main() {
    foo.do_something(foo.pi)
}
```

## Example 2

The following program imports `foo/bar.kasl` in the search path.

`foo/bar.kasl`
```kasl
let e = 2.71828
```

`main.kasl`
```kasl
import foo/bar

output e = 0.0

func main() {
    e = bar.e
}
```
