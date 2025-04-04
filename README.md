# easyjs
easyjs is a new programming language built for personal reasons.
I have learned a multitude of programming languages and I really enjoy programming. I enjoy learning new languages and applying the ideas and constructs
in others. easyjs is my own personal language that I can expand as I please. It lives ontop of JS (hence the js in the name) and compiles using Rust.
I've never been a fan of JS but it has a huge ecosystem and runs natively in the browser.

## 2 things
2 Things I want easyjs to accomplish for myself is:
1. Be an easy scripting programming language with a rich standard library.
2. Run natively in the web for easy sharing.

> [!WARNING]  
> This language is still far from v1.0.0 we are currently on v0.2.1

## Install
To install you have a few options.

### Download
Downloand and run the installer at [easyjs](https://github.com/grupojvm/easyjs)

### GIT
Clone this Git repo and run `cargo build --release` to build the binary. It does not take long to build since easyjs only has 4 dependencies.

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
easyjs
> // your code goes here.
```

### Examples
Imagine you have a easyjs file like so:
```rust
fn foo() { // <-- functions use the fn keyword.  this will compile into a "function foo() {}"
    console.log("foo") // <-- mostly all JS objects  transfer over. 
}

bar = fn(x,y) {  // <-- This will compile into a "const bar = () => {};"
    ...
}
```
You can compile this using our easyjs CLI.
`easyjs compile file.ej` --> this will transcribe to a file.js

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
In this approach our wasm runtime will take care of transcribing it in REALTIME.

**Fibonacci**
```rust
fn fibonacci(n):int { // <-- easyjs is optionally typed. 
    if n == 0 {
        return 0 // <-- no semicolons.
    } elif n == 1 {
        return 1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2) // when typed you can default the last statement to being returned.
    }
}
```
**Manipulating the DOM**
```rust
document.title = "Hello World!" // <-- No semicolons

// I know this is not much but easyjs will have a dedicated dom api in version 1.0.0
```
VS the JavaScript equivalent

That's a pretty basic example, but you can already tell it is a little more readable without the semicolons.

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
**Classes and objects**
```rust
// easyjs does not currently support classes. Only data structs.
// classes will be added by v1.0.0

// [name,age] are values that are passed into the constructor.
pub struct Person[name, age] with GreetMixin {
    var has_job = true
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
        console.log('Hello, my name is ${self.name}')
    }
}

// Structs can be with dedicated methods or just simple data containers
struct PersonData[name,age,diary] {} // <--a struct that accepts name, age, and diary.

// To instantiate a Person
person = Person("Jordan", 22, ["Dear Diary", "I love Julia!", "I also love EasyJS!"])

// To instantiate a PersonData
person_data = PersonData("Evelyn", 19, ["Dear Diary", "I saw that Jordan loves a girl named Julia!", "Who is she???"])
```

**Variables**
```javascript
var hello = "hello" // this compiles into let hello = "hello"
world = "world" // this compiles into const world = "world"

// Why does const not need to have a `const` keyword? Because I don't like it.

// easyjs optional typing
var helloTyped : string = "hello"
helloTyped = 1 // this will error during compilation
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

## Built with easyjs
Here is a list of projects using easyjs.

- easysj[.]
- pixel_ai_dash_web