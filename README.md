# easyjs
easyjs is a new programming language built for fun.
I have learned a multitude of programming languages and I really enjoy programming. I enjoy learning new languages and applying the ideas and constructs
in others. easyjs is my own personal language that I can expand as I please. It lives ontop of JS (hence the js in the name) and compiles using Rust.
I've never been a fan of JS but it has a huge ecosystem and runs natively in the browser.

## 2 things
2 things easyjs is built to accomplish is:
1. Be an easy scripting programming language with a rich standard library.
2. Run natively in the web for easy sharing.

## ECMAScript version
Easyjs uses the ECMAScript 2020 version (ES11). 
This means that new features being added to ECMAScript will not be oficially supported. But a smart person could include them in their project 
using macros, templates and the `javascript{}` statement.

> [!WARNING]  
> This language is still in development we are currently on v0.4.5

## Install
To install you have a few options.

### Download
Download from releases.

### GIT
Clone this Git repo and run `cargo build --release` to build the binary.

## How to use
You have many different options to use. 

**Compile:**
You can compile easyjs to a js file to run on the browser, server, etc.
```bash
easyjs compile file.ej
```

**Script tag:**
You can use a `<script type="easyjs">` tag in the browser to inline the easyjs. <-- This requires the easyjs wasm runtime.

You can use a `<script src="source.min.js">` tag in the browser.

**REPL**
easyjs provides a REPL. Use it by running `easyjs` in your terminal.
```bash
easyjs repl
> // your code goes here.
```

You can use any of the following runtimes
- node
- deno
- easyjsr (this is the default, but is currently lacking in some features)

**Online:**
You can also go to the (easyjs website)[https://jordanmcastro.com/easyjs]

### Examples
Imagine you have a easyjs file like so:
```rust
fn foo() { // <-- functions use the fn keyword.  this will compile into a "function foo() {}"
    console.log("foo") // <-- mostly all JS objects  transfer over. 
}

bar = fn(x,y) {  // <-- This will compile into a "let bar = () => {};"
    ...
}
```
You can compile this using
`easyjs compile file.ej` --> this will create a file.js

Or you can inline the .ej file
```html
<head>
    <script src="file.ej" type="easyjs"></script>
    <!-- OR -->
    <script type="easyjs">
        fn foo() {
            console.log("foo")
        }

        fn bar() {
            console.log("bar")
        }
    </script>
</head>
```
In this approach our wasm runtime will take care of compiling it in REALTIME.

**Fibonacci**
```rust
fn fibonacci(n):int { // <-- easyjs is optionally typed. 
    if n == 0 {
        return 0 // <-- no semicolons.
    } elif n == 1 {
        return 1
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2) 
    }
}
```

**Making a GET request**
```js

async { // optionally wrap in a async block if you want to use await
    get_response = await fetch("https://jsonplaceholder.typicode.com/posts/1")
    if get_response.status_code == 200 {
        @print(get_response.json()) // a builtin macro
    } else {
        // a javascript inliner
        // this is useful because easyjs lacks error handling and exception throwing.
        javascript {
            throw new Error("Network response was not ok");
        }
        // But ideally you would not throw an error but instead log
        console.error("Network resposne was not ok");
    }
}
```

**Objects**

Structs are data first objects.
```rust
// [name, age] are values that are passed into the constructor.
struct Person[
    name:string, 
    age:int
] with GreetMixin {
    // Static
    has_job = true
    species = "HomoSapien"

    fn set_name(self, new_name) {
        self.name = new_name
    }

    /// gets the name. // <-- doc comments with '///'
    fn get_name(self) {
        return self.name
    }

    // this is a static method because it does not have self as a paramater.
    fn static_method() {
        console.log("This is a static method")
    }
}

// A mixin is just another struct
struct GreetMixin {
    fn say_hi(self) {
        console.log("Hello, my name is ${self.name}")
    }
}
// Structs can be with dedicated methods or just simple data containers
struct PersonData[
    name,
    age,
    diary
] {} // <--a struct that accepts name, age, and diary.

// To instantiate a Person
person = Person("Jordan", 22, ["Dear Diary", "I love Julia!", "I also love EasyJS!"])

// To instantiate a PersonData
person_data = PersonData("Evelyn", 19, ["Dear Diary", "I saw that Jordan loves a girl named Julia!", "Who is she???"])
```
Classes compile directly to JS classes. Also include multiple inheritance and private/public fields.
```js
class A {
    // __new__ is for constructor
    fn __new__(self) {
        super()
        @print('A')
    }

    // Public method
    pub fn foo(self) {
        @print('Foo in A')
    }

    // Private method
    fn bar(self) {
        @print('Private foo in A')
    }

    // Calling a private method
    pub fn call_priv(self) {
        self.bar()
        // Automatically converts to:
        // this.#bar()
    }
}

class B {
    fn __new__(self) {
        super()
        @print('B')
    }
}

// easyjs classes support multiple inheritance
class C : [A, B] {
    fn __new__(self) {
        super() // Don't forget to call super!
        @print('C')
    }
}
```

**Variables**
```javascript
hello = "Hello"

// If we want to do a const we have to use the @const macro from 'std'
import 'std' as _
@const(world = "World")

// easyjs optional typing
hello_typed : string = "hello"
```

**Macros**
easyjs includes macro support allowing developers to build their own feature rich DSLs.
```rust
// for example the const macro in 'std'
macro const(expr) {
    // All easyjs has access to the javascript statement.
    // This is a statement that allows you to place literal unparsed code into a context.
    // This should be used very carefuly.
    javascript{
        const #expr; // notice we need to use '#' symbol to access macro paramaters
    }
}

// print macro in 'std'
macro print(...s) {
    console.log(s)
}
```

**Native**
easyjs supports a builtin wasm compiler named `easyjs native`. To use the wasm compiler wrap your code in a `native` block.
```rust
native {
    // native functions need to be typed.
    pub fn add(n1:int, n2:int):int {
        n1 + n2
    }
}

// then to call the built function
result = add(1,2)
@print(result)
```
Yes it is that easy!

## Features
easyjs contains features that are important in a programming language (to me atleast!).
1. Easy to read and write.
2. Optional typing, sometimes you don't want types.
3. Fast scripting language with high performance support.

<!-- ## Built with easyjs
Here is a list of projects using easyjs.

- The Pixel Game Engine: a game engine optomized for mobile builds that uses easyjs as it's scripting language. -->