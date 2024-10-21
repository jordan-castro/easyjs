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

## I think the main thing is
EasyJS is only meant to be used in the browser. It is not general purpose and is meant only to be used within the browser.
I can not stress this enough. It is only meant to be used within the browser. 

## How I see this going
I see EasyJS being used in place of JS/TS in a lot of places. It will be easier and faster to use the intuitve syntax of EasyJS to write
frontend libraries, machine learning, and complex algorithms. Because EasyJS is esentially Vanilla JS at the end of the day, it can be 
incorporated easily with frontend JS. EasyJS can very well be used for whole web applications as well.