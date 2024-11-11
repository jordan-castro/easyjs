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
EasyJS supports macros. To define a macro or to call one use the `$` symbol.
```rust
fn $print(message) { // this is the $print macro.
    console.log(message)
}

$print("Hello World!") // <-- call the print macro, this is compiled to console.log("Hello World!")
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