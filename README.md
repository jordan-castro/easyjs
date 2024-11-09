# EasyJS
EasyJS is a new programming language which makes it easy to write web applications easily and naturally. 
Using a modern like syntax to interact FULLY with the DOM, server, and anywhere else that JS runs. 

EasyJS works by compiling down into efficient and performant JavaScript.
Similar to CofeeScript and TypeScript. The main difference being that it is an easy to use language, and can run natively on web.

> [!WARNING]  
> This language is still far from v1.0.0 we are currently on v0.1.4

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
    print("foo") // <-- This will console.log("foo")
    // or you could just use
    console.log("foo") // <-- mostly all JS objects  transfer over. 
}

bar := fn(x,y) {  // <-- This will compile into a "const bar = () => {};"
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
            print("foo")
        }

        fn bar() {
            print("bar")
        }
    </script>
</head>
```
In this approach our wasm runtime will take care of transcribing it in REALTIME.

**Fibonacci**
```rust
fn fibonacci(n) { // <-- easyJS is dynamically typed. 
    if n == 0 {
        return 0 // <-- optional semicolons.
    } elif n == 1 {
        1 // <-- optinal return statement.
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```
VS the JavaScript equivalent
```javascript
function fibonacci(n) {
    if (n === 0) {
        return 0;
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
```
VS the JavaScript equivalent
```javascript
document.title = "Hello World!";
```

That's a pretty basic example, but you can already tell it is a little more readable without the semicolons.

**Making a GET request**
```rust
import http // <-- import the easyjs http library

get_response := get("https://jsonplaceholder.typicode.com/posts/1") // <-- Call the http.get method
get_response.if { // <-- Conditional on object type.
    .status_code == 200 { // <-- if get_response.status_code == 200 (you also can use .ok which does the same thing)
        console.log(.json())
    } else {
        throw(NetworkError) // <-- You can also use a general Error(msg)
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
struct Person {
    pub name::string // <-- Optinal typing
    pub age // <-- struct properties are private by default.
    diary::array[string] // <-- This property is private and is an array of strings.

    pub fn _new(name, age, diary) {} // <-- If using the same name as the struct property it will be set automatically.

    pub fn say_greeting() {
        console.log("Hello, my name is $name") // <-- Example of string interpolation. No need for ``
    }

    fn read_diary() { // <-- struct functions are private by default.
        for item of diary { // <-- 
            print(item)
        }
    }
}

// You also have the option of a methodless struct for holding data
struct PersonData {
    pub name
    pub age
    pub diary
}

// To instantiate a Person
person := Person("Jordan", 22, ["Dear Diary", "I love Julia!", "I also love EasyJS!"])

// To instantiate a PersonData
person_data := PersonData("Evelyn", 19, ["Dear Diary", "I saw that Jordan loves a girl named Julia!", "Who is she???"])
```
VS the JavaScript equivalent
```javascript
class Person {
    constructor(name, age, diary) {
        this.name = name;   // Name of the person
        this.age = age;     // Age of the person
        this.diary = diary; // List of strings for the diary
    }

    // Public method to say a greeting
    sayGreeting() {
        console.log(`Hello, my name is ${this.name}`);
    }

    // Private method to read the diary
    #readDiary() {
        this.diary.forEach((entry, index) => {
            console.log(`Diary entry ${index + 1}: ${entry}`);
        });
    }
}

// A object in JS
function PersonData(name, age, diary) {
    this.name = name;
    this.age = age;
    this.diary = diary;
}

// To instantiate a Person
const person = new Person("Jordan", 22, ["Dear Diary...", "..."])

// To instantiate a PersonData
const personData = new PersonData("Evelyn", 19, ["Dear Diary", "..."])

```
**Variables**
```php
const_var := "some data"                             // <-- This is a const string variable. consts are typed automatically.
variable = "other data"                              // <-- This is a mutable string variable.
typed_var::string = "more data"                      // <-- This is how you would type a variable.
global variable = "a global string"                  // <-- This is a dynamic global mutable variable.
global static_var::string = "a static global string" // <-- This is a static global mutable variable.
```
VS the JavaScript equivalent
```javascript
const constVar = "some data"     // JS equivalent
let variable = "other data"      // JS equivalent
                                 // no typed option...
var variable = "a global string" // global variable equivalent
```

## I think the main thing is
I'm building EasyJS to run wherever JavaScript runs, this is because it compiles into js. That means you could in theory use it with node, bun, deno, on the browser, apps, ect. The whole idea is to make an easier, modern, intuitive language that replaces nasty JS.

## What's wrong with JS?
A lot of things, but to get started the main things which every JS developer will mention
 hard to read syntax, easily error prone, no types, and strange behavior. EasyJs is focused on fixing 3 of these headaches.
 1. Easy to read and modern syntax.
 2. Catches errors before it hits the runtime.
 3. Optional typing.

## How I see this going
I see EasyJS being used in place of JS/TS in a lot of places. It will be easier and faster to use the intuitve syntax of EasyJS to write
frontend libraries, machine learning, and complex algorithms. Because EasyJS is esentially Vanilla JS at the end of the day, it can be 
incorporated easily with frontend JS. EasyJS can very well be used for whole web applications as well.