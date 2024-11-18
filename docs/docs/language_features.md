# Language Features

As a modern language, EasyJS supports a lot of modern features. We try to support as much as to make JavaScript developers lives easier, while staying clean and easy to read/write.

## Features

### Functions
In easyjs to define a function you just need to use the `fn` keyword.
```rust
fn foo(arg1, arg2, ...) {
    // code goes here...
}
```

### Lambda Functions
In easyjs to define a lambda function you just need to use the `fn` keyword without a function name.
```rust
bar = fn(arg1, arg2, ...) {
    // code goes here...
}
```

### Variable Decleration
EasyJS supports constant and non constant variable decleration.
```rust
// non constant -> compiles into let
foo = "bar"
// constant -> compiles into const
bar := "foo"
```
In easyjs to define a constant variable you just need to use the `:=` keyword.

### If statements
```rust
if condition {

} elif other_condition {
    
} else {

}
```

### Loops
In easyjs the only loop is a for loop.
```rust
// range loops
for i in 0..10 {
    // code goes here...
}

// while loops
for condition {
    // code goes here...
}

// foreach loops
for item in collection {
    // code goes here...
}
```

### Macros
EasyJS supports macros. Macros in EasyJS are a form of metaprogramming allowing you to host your own code in a virtual enviroment.
Macros work in 2 ways:

- DunDun (Interpreter)
    - Using the `$` symbol before a macro call means that this macro is interpreted and the last result is returned.
- Decorator (Transpiles)
    - Using the `@` symbol before a macro call means that this macro is transpiled into Javascript as other EasyJS functions. The benefit
    of using the `@` is it allows you to hoist function names away from the end JS scope while still being regular easyJS methods.

The benefir of both macro types is that they are hosted in a virtual enviroment are are only accessible via easyJS.


Macros are defined using the `macro` keyword.
```rust
macro print(message) { // this is the $print macro.
    console.log(message)
}

@print("Hello World!") // This will be transpiled into --> console.log("Hello World!")
$print("Hello World!") // While being a valid macro call, this will not produce any result because nothing is being returned.

macro on_compile_do_stuff() {
    ...
}

$on_compile_do_stuff() // <-- This will run directly in the compiler enviroment.
```

### JS Objects
As easyjs is a easier version of JS, it supports JS objects.
```javascript
// JSON
person := {
    name: "Jordan Castro",
    age: 22,
    occupation: "Creator of EasyJS"
}

// ARRAYS
numbers := [1, 2, 3, 4, 5]
```

### Strings
EasyJS supports single quoute `'` and double quote `"` strings.
```dart
hello = "Hello"
world = "World"

$print("$hello, $world!") // <-- how to interpolate.
```

### Structs
In easyjs you use structs to create and define objects. Structs currently support

- Methods
- Constructor
- Static Methods

```rust
struct Person {
    fn new(name, age) {
        self.name = name
        self.age = age
    }

    fn greet(self) {
        @print(self)
    }

    fn a_static_method() {
        @print("Ayo sttaic dude")
    }
}
```
To instantiate a struct object you need to use the `javascript` keyword.
```javascript
javascript{
    const person = new Person("jordan", 22);
    person.greet();
    Person.a_static_method();
}
```
This is temporary and will be changed in future versions.