# easyjs Design Doc
The easyjs design document. Documenting what will be easyjs going forward.

## KISS
Keep it simple stupid. In easyjs there are many ways to do the same thing. But here is what we recommend.

### Modules
In easyjs there are no namespaces or modules.

But to act as "pseudo" modules you can:
```js
// Use a struct. This is the best way to handle most modules in easyjs.
// Make sure you add pub, otherwise the name will be mangled
pub struct Math {
    fn add(...numbers):int {
        result = 0
        for n in numbers {
            result += n
        }

        result
    }
}
// Or add a prefix to your methods
pub fn math_add(...numbers):int {
    result = 0
    for n in numbers {
        result += n
    }
    result
}

// If doing this for multiple functions you can use the `prepend` macro

prepend! math {
    pub fn sub(...numbers):int {
        result = 0
        for n in numbers {
            result -= n
        }
        result
    }
}

// All easyjs code is eventually compiled to JS and completely exposed to JS. 
// When you don't add `pub` the name will be mangled.
fn internal_sub(...numbers):int {
    // ...
}

// You can all `internal_sub` normally within non native easyjs only.
// Does not get exposed to native easyjs due to JS mangling.

// If you want to support ES11 exports use the export! macro
export! math_sub
export! math_add

// You can also just add it before the function decleration
export! pub fn math_div(...numbers):float {
    // ...
}

// The same is for variables
export! pub math_PI = 3.145678
```
When using this "math.ej" file as a module.
```js
import "math.ej"

// You can use the `::` sugar token to automatically prepend left side onto any right side with a '_' in between.
print!(math::add(1,2)) // 3
```

|symbol|purpose|target|type|
|------|-------|------|----|
|export!|To support ES11 module exports.|JS|macro|
|pub|To disable name mangling within easyjs and JS.|JS and EJ|keyword|


### Objects/Structs
Because easyjs is JS at the end of the day and wants to interop perfectly, it supports objects, and easyjs structs.
You should use structs in all new code.
```js
// Example of old JS way
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }

    greet() {
        console.log("Hello my name is: " + this.name + " and I am " + this.age.toString() + " years old!");
    }
}

let p = new Person("Jordan", 24);
p.greet()

// New easyjs way
pub struct Person[
    name:string, // types are optional
    age:int
] {
    // Pub means the name will not be mangled. 
    // Both struct and fn need to be pub to avoid mangling
    pub fn greet(self) {
        print!()
    } 
}

p = Person("Jordan", 24)
p.greet()

// You do have backards compat for
p = {
    name: "Jordan",
    24
}
```

## Native
easyjs can compile `native{}` blocks into WASM modules that expose functions and variables to `non native` easyjs scope.

This is where easyjs stops being easy. The `native` scope can get very complex. We recommend you follow these guidelines.

### Modules
All `native` code gets compiled into one module. Because of that there is no name mangling in `native` mode. 

And the `pub` keyword instead exposes to the `non native` context what is available for use.

When handling 3rd party libraries it is important to prepend the library name to all methods and structs. You can then use the `::` sugar token to prepend the 
library name before functions.

### Objects/structs
`native` only supports structs. It does not support js raw objects.

### Declaring public variables, methods
```js
native {
    // If you want to use this in non native easyjs
    pub PI = 3.1456

    // Methods
    pub fn add(...numbers:int[]):int {
        numbers.add()
    }

    fn sub(...numbers:int[]):int {
        numbers.sub()
    }
}

// Calling from easyjs
pi = PI

print!(add(pi, 2)) // 5

catch_print! print!(sub(0,1,2,3)) // prints: "Exception: sub is not public" 
```

## Macros
Only currently supporting text based and hygenic formats.
```js
// A simple text based macro is the builtin print! macro
macro print(...args) {
    console.log(#args) // requires using '#' for macro params
}

print!("Hello", "World", "!") // compiles: console.log("Hello", "World", "!")
                              // prints: Hello World !

// A hygenic macro uses `{{}}`
// This would be a example of print but hygenically
macro hprint(...args) {{
    string = ""

    for arg in [#args] {
        string += arg + " "
    }

    "
javascript { 
    console.log($string); 
}
    "
}}

hprint!("Hello", "Hygenic") // compiles: console.log("Hello Hygenic ");
                            // prints: Hello Hygenic 

```

## Error handling
`error` is a builtin type. It can be returned from any method typed with a `?` at the end.
```js
// This is the http library
prepend! http {
    pub async fn get(url, params): Response? {
        // ...
    }
}

// When calling http::get you would have to check if your result is == error.
result = await http::get("UTES")
if result == error {
    print!("Got an error")
}

// Or you can use the macro `is_error`
if is_error!(result) {
    print!("Got an error")
}

// You can also use a on_eror macro
// This macro is mostly for default values on error.
result = on_error!(await http::get("UTES"), "Oh helll nah!")
if result == "Oh helll nah!" {
    print!("Got an error")
}

// This does not work with RAW js
result = await fetch("UTES")
// Exception thrown. Did not catch because RAW js.

// To catch it you need to wrap in a try catch
// Works the same way as on_error! but with JS exceptions. on_error checks for `error` type, not exceptions.
result = try_catch!(await fetch("UTES"), 0)
if result == 0 {
    print!("Got an error")
}
```