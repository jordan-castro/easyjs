# EasyJS
EasyJS is a new programming language which makes it easy to write web applications easily and naturally. 
Using a modern like syntax to interact FULLY with the DOM, server, and anywhere else that JS runs. 

EasyJS works by compiling down into efficient and performant JavaScript.
Similar to CofeeScript and TypeScript. The main difference being that it is an easy to use language, and can run natively on web.

> [!WARNING]  
> This language is still far from v1.0.0 we are currently on v0.2.1

## Why EasyJS?
At my last job, I worked with a product that would track user engagement on websites and apps. At this job I realized how common it was for people to not know
how JavaScript works and get lost in it's syntax. At the same time, I simply dislike working in vanilla JS. And TS just adds to much boilerpate. Too much unecessary boilerplate. So this idea came to mind.

Keep in mind that the first few versions of EasyJS will probably be extremely slow and very basic. It's probably not going to work very well to be honest (at first).

## Install
To install you have a few options.

### Download
Downloand and run the installer at [easyjs](https://github.com/grupojvm/easyjs)

### GIT
Clone this Git repo and run `cargo build --release` to build the binary. It does not take long to build since easyjs only uses 2 dependencies.

## How to use
You have many different options to use. 

**Compile:**
You can compile easyJS to min.js to run on the browser, server, etc.
```bash
easyjs compile file.ej
```

**Script tag:**
You can use a `<script type="easyjs">` tag in the browser to inline the easyJS. <-- This requires the easyjs wasm runtime.

You can use a `<script src="source.min.js">` tag in the browser.

**REPL**
EasyJS provides a REPL. Use it by running `easyjs` in your terminal.
```bash
easyjs
> // your code goes here.
```

### Examples
Imagine you have a EasyJS file like so:
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
fn fibonacci(n):int { // <-- easyJS is optionally typed. 
    if n == 0 {
        return 0 // <-- no semicolons.
    } elif n == 1 {
        return 1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2) // when typed you can default the last statement to being returned.
    }
}
```
VS the JavaScript equivalent
```javascript
function fibonacci(n) { // no types (what??)
    if (n === 0) {
        return 0; // always need return
    } else if (n === 1) {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

```
**Manipulating the DOM**
```rust
document.title = "Hello World!" // <-- No semicolons

// I know this is not much but easyjs will have a dedicated dom api in version 1.0.0
```
VS the JavaScript equivalent
```javascript
document.title = "Hello World!";
```

That's a pretty basic example, but you can already tell it is a little more readable without the semicolons.

**Making a GET request**
```js

async { // optionally wrap in a async block if you want to use await
    get_response = await fetch("https://jsonplaceholder.typicode.com/posts/1")
    if get_response.status_code == 200 {
        @print(get_response.json()) // a builtin macro
    } else {
        // a javascript inliner
        // this is useful because not all of JS is currently supported (like exceptions...)
        javascript {
            throw new Error("Network response was not ok");
        }
    }
}
```
VS the JavaScript equivalent
```javascript
fetch('https://jsonplaceholder.typicode.com/posts/1')
    .then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.json(); // Parse the response as JSON
    })
    .then(data => {
        console.log(data); // Handle the JSON data here
    })
    .catch(error => {
        console.error('There was a problem with the fetch operation:', error);
    });
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
// To instantiate a Person
person = Person("Jordan", 22, ["Dear Diary", "I love Julia!", "I also love EasyJS!"])

// To instantiate a PersonData
person_data = PersonData("Evelyn", 19, ["Dear Diary", "I saw that Jordan loves a girl named Julia!", "Who is she???"])
```
VS the JavaScript equivalent
```javascript
function Person(name, age) {
    Person.species = 'HomoSapien';
    Person.static_method = function () {
        console.log('This is a static method');
    };
    return Object.assign({
        name, age, has_job: true, set_name: function (new_name) {
            this.name = new_name;
        },
        get_name: function () {
            return this.name;
        },
    }
        , GreetMixin(),);
}
function GreetMixin() {
    return {
        say_hi: function () {
            console.log(`Hello, my name is ${self.name}`);
        },
    }
}

// To instantiate a Person
const person = new Person("Jordan", 22, ["Dear Diary...", "..."])

// To instantiate a PersonData
const personData = new PersonData("Evelyn", 19, ["Dear Diary", "..."])

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
VS the JavaScript equivalent
```javascript
let variable = "other data"      // JS equivalent
const constVar = "some data"     // JS equivalent

// no typed equivalent.
```
**Native (wasm)**
easyjs supports a builtin wasm compiler named `easyjs native`. To use the wasm compiler wrap your code in a `native` block.
```rust
native {
    // native functions need to be typed.
    // paramaters need to be typed but function returns do not need to be typed.
    // if they are not typed though, you loose some wasm features.
    pub fn add(n1:int, n2:int):int {
        n1 + n2
    }

    // untyped version
    pub fn add_untyped(n1:int, n2:int) {
        return n1 + n2 // basically you would need to write out return specifically.
    }
}

// then to call the built function
result = add(1,2)
result_typed = add_untyped(1,2)
```
Yes it is that easy!

## I think the main thing is
I'm building easyjs to run wherever JavaScript runs, this is because it compiles into js. That means you could in theory use it with node, bun, deno, on the browser, apps, ect. The whole idea is to make an easier, modern, intuitive language that can replace JS.

## What's wrong with JS?
A lot of things, but to get started the main things which every JS developer will mention
 hard to read syntax, easily error prone, no types, and strange behavior. easyjs is focused on fixing 3 of these headaches.
 1. Easy to read and modern syntax.
 2. Catches errors before it hits the runtime.
 3. Optional typing.

## How I see this going
I see easyjs being used in place of JS/TS in a lot of places. It will be easier and faster to use the intuitve syntax of EasyJS to write
frontend libraries, machine learning, and complex algorithms. Because EasyJS is esentially Vanilla JS at the end of the day, it can be 
incorporated easily with frontend JS. EasyJS can very well be used for whole web applications as well.