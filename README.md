# EasyJS
EasyJS is a new transpiled programming language which makes it easy to write web applications easily and naturally. 
Using a modern like syntax to interact FULLY and ONLY with the DOM. 

Transpiled means that it goes from one language to another, it is not compiled nor interperted. It goes from EasyJS to JS. 
Similar to CofeeScript and TypeScript. The main difference being that it is an easy to use language.

> [!WARNING]  
> This language is not ready to be used. We are not even on version 0.0.1 yet.

## Why did I make this?
I made this because frankly I dislike working in vanilla JS. And TS just adds to much boilerpate. Too much unecessary boilerplate. So this idea came to mind.
The first few versions of EasyJS will probably be extremely slow and very basic. It's probably not going to work very well to be honest (at first).

## How to use
You have many different options to use. You can precompile it before adding it to the browser. 
Or run inline using WASM. <-- This approach while cool is really only meant for quick debugging.

### Examples
Imagine you have a EasyJS file like so:
```rust
fn foo() {
    print("foo") // <-- This will console.log("foo")
}

fn bar() {
    print("bar") // <-- This will console.log("bar")
}
```
You can compile this using our easyjs CLI.
`easyjs compile file.ej` --> this will transcribe to a file.min.js

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
The above approach should really be used only for debugging.

**Fibonacci**
```rust
fn fibonacci(n) { // <-- easyJS is dynamically typed with (optinal typing).
    if n == 0 {
        return 0 // <-- easyJS has an optional return keyword
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```
VS the JavaScript equivalent
```javascript
const fibonacci = (n) => {
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
title := get_element_by_tag("title")
title.text = "Hello World!"
```
VS the JavaScript equivalent
```javascript
const title = document.getElementsByTagName("title")[0];
title.text = "Hello World!"
```

**Making a GET request**
```rust
import http // <-- import the easyjs http library

get_response := http.get("https://jsonplaceholder.typicode.com/posts/1")
get_response.if { // <-- Conditional on object type.
    .status_code == 200 { // <-- if get_response.status_code == 200 (you also can use .ok which does the same thing)
        print(.json())
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
        print("Hello, my name is $name") // <-- Example of string interpolation. No need for ``
    }

    fn read_diary() { // <-- struct functions are private by default.
        diary.for(item) { // <-- foreach loop.
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
```rust
const_var := "some data" // <-- This is a const string variable. consts are typed automatically.
var = "other data" // <-- This is a mutable string variable.
typed_var::string = "more data" // <-- This is how you would type a variable.
global var = "a global string" // <-- This is a dynamic global mutable variable.
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
EasyJS is only meant to be used in the browser. It is not general purpose and is meant only to be used within the browser.
I can not stress this enough. It is only meant to be used within the browser. 

## How I see this going
I see EasyJS being used in place of JS/TS in a lot of places. It will be easier and faster to use the intuitve syntax of EasyJS to write
frontend libraries, machine learning, and complex algorithms. Because EasyJS is esentially Vanilla JS at the end of the day, it can be 
incorporated easily with frontend JS. EasyJS can very well be used for whole web applications as well.