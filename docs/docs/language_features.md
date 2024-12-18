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
```js
// mutable -> compiles into let
var foo = "bar"
// constant -> compiles into const
bar = "foo"
```
In easyjs all variable declerations are considered constants to the extent that it's not told otherwise.

### If statements
```js
if condition {
    console.log("if")
} elif other_condition {
    console.log("else if")
} else {
    console.log("else")
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
<!-- EasyJS supports macros. Macros in EasyJS are a form of metaprogramming allowing you to host your own code in a virtual enviroment.
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

@print("Hello World!") // This will be transpiled into console.log("Hello World!")
$print("Hello World!") // While being a valid macro call, this will not produce any result because nothing is being returned.

macro on_compile_do_stuff() {
    ...
}

$on_compile_do_stuff() // <-- This will run directly in the compiler enviroment.
``` -->

### JS Objects
As easyjs is a easier version of JS, it supports JS objects.
```js
// JSON
person = {
    name: "Jordan Castro",
    age: 22,
    occupation: "Creator of EasyJS"
}

// ARRAYS
numbers = [1, 2, 3, 4, 5]
```

### Strings
EasyJS supports single quoute `'` and double quote `"` strings.
```dart
hello = "Hello"
world = "World"

console.log("$hello, $world!") // <-- how to interpolate.

// multiline string
multi_line = "
This
is 
a 
multi
line
string
"
```

### Structs
In easyjs you use structs to create and define objects. Structs currently support

- Methods
- Constructor
- Static Methods

```rust
struct Person {
    // You can define the constructor like that or
    fn constructor(name, age) {
        self.name = name
        self.age = age
    }
    // like this
    fn new(name, age) {
        self.name = name
        self.age = age
    }

    fn greet(self) {
        console.log(self)
    }

    fn a_static_method() {
        console.log("Ayo sttaic dude")
    }
}
```
To instantiate a EasyJS struct object you do so like:
```js
person = Person('Jordan Castro', 22)
```
There is no need for the `new` keyword.

But if you are using JS objects which means objects that were not defined in EasyJS you need to use the `new` keyword.
This is temporary and will be changed in future versions.

#### Composition
EasyJS structs are built with compisition in mind rather than inheritance. This means that to create a President struct you might do:
```js
struct President {
    var person = null

    fn new(name, age, years_in_office) {
        self.person = Person(name, age)
        self.years_in_office = years_in_office 
    }
}

president = President("Donald Trump", 78, 4)
president.person.greet()
```
While composition is preffered, there are times where inheritance is valuable and easyjs does support it as well.

#### Inheritance
While easyjs structs are built with composition in mind rather than inheritance, there are times where inheritance is valuable.
```js
struct President(Person) {
    fn new(name, age, years_in_office) {
        super(name, age) // super the constructor.
        self.years_in_office = years_in_office
    }
}

president = President("Donald Trump", 78, 4)
president.greet()
```

### Kwargs
In EasyJS we support the ability to pass named arguments to functions in a special manner. Take the function below
```rust
fn add(kwargs) {
    return kwargs.a + kwargs.b
}
```

To call this function you would use named parameters like:
```py
    add(a=1, b=2)
```

And this gets compiled into:
```js
    add({a:1,b:2});
```

### Importing modules
To import modules in easyjs you use the `use` keyword. There are 4 different types of imports that easyjs allows.

1. base
2. core
3. js
4. string

As for which type of import to use,

For importing easyjs files that are relative to the current script:
```rs
use base:path.to.script

// for importing specific functions/structs
use {add} from base:path.to.add.script
```

For importing the easyjs std library:
```rs
use core:builtins
```

For a full list of easyjs core library check out: this link

For importing JS files:
```rs
use js:file.path
```

For using npm packages or DENO imports:
```js
use string:"npm package name"
// DENO
use string:"jsr:package name"
use string:"npm:package name"
use string:"https://url.js"
use string:"some_file.wasm"
```
EasyJS does not strictly support importing WASM files. 