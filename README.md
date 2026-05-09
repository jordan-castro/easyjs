# easyjs
easyjs is a general purpose programming language that compiles to JS and WASM.
JavaScript has a huge ecosystem and runs natively in the browser.
JS alongside with WASM, macros, and types becomes really powerful.

## ECMAScript version
easyjs uses the ECMAScript 2020 version (ES11). 
This means that new features being added to ECMAScript will not be officially supported. But a smart person could include them in their project 
using macros and the `javascript{}` statement.

> [!WARNING]  
> This language is still in development we are currently on v0.5.0

## Install
To install you have a few options.

### Download
Download from releases.

### GIT
Clone this Git repo and run `cargo build --release` to build the binary.

## How to use
You have many different options to use. 

### Compile
You can compile easyjs to a js file to run on the browser, server, etc.
```bash
easyjs file.ej file.js
```

### Script tag
You can use a `<script type="easyjs">` tag in the browser to inline the easyjs. <-- This requires the easyjs wasm runtime.

You can use a `<script src="source.js">` tag in the browser.

### Repl
easyjs provides a REPL. Use it by running `easyjs` in your terminal.
```bash
easyjs
> // your code goes here.
```

You can use any of the following runtimes
- node
- deno
- ejr (this is the default, but is currently lacking in some features)

### Online
You can also go to the [easyjs website](https://jordanmcastro.com/easyjs)

### easyjs runtime
easyjs has it's own custom runtime [ejr](https://github.com/jordan-castro/ejr). It's a low level implementation largely inspired by [quickjs](https://bellard.org/quickjs/) with focus on top notch wasm integration. It is currently lacking features and targets [ECMAScript 2020 (ES11)](https://tc39.es/ecma262/2020/).

Run a file directly with:
`easyjs file.ej`

## Key features
- Modern syntax.
- Optional typing.
- Wasm compilation targets.
- Hygienic/text injection macros.

## Example
Imagine you have a easyjs file like so:

```js
// ================ Variables ================
// To declare immutable variables
hello := "World"

// To declare mutable variables
world = "Hello"

// Type variable
number : int = 0
decimal : float = 0.5

// ================ Functions ================
fn sum_numbers(numbers: []int):int {
    return (numbers / 2) * ((numbers / 2) + 1);
}

print!(sum_numbers([1,2,3])); // 6

// ================ Objects ================
// Defines a Prototype
struct Person {
    name: string,
    age: int,
}

// Defines object constructor 
fn Person.new(name:String, age:int): Person {
    return Self {
        name,
        age
    }
}

// Instantiate the prototype into a object
person := Person.new("Jordan", 24)
// macro expands to `console.log(...args)` 
print!(person.name)
print!(person.age)

// ================ Wasm ================
// compiles to WASM bytecode.
#wasm fn add(n1:int, n2:int):int {
    return n1 + n2
}

// Call wasm function
print!(#wasm.add(1,2)) // 3

// ================ Macros ================
#hmacro fn write_function(name) {
    // Hygenic
    return "fn $name() "
}

#macro fn eprint(...args) {
    // Text injection
    console.error(...args)
}

write_function("eprint_hello_world") {
    eprint!("Hello World!")
}

print_hello_world!()
```

Yes it is that easy!