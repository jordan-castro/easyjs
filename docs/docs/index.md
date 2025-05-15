# EasyJS Documentation

Welcome to the official documentation for **EasyJS**, a modern, beginner-friendly programming language that compiles down to JavaScript. EasyJS makes it intuitive and efficient to build web applications by providing clear, minimal syntax and extensive features for interacting with the DOM, server, and other JavaScript environments.

> [!WARNING]
> EasyJS is currently in an early stage of development and has yet to reach version `1.0.0`. The latest release is `v0.4.2`.

---

## Why EasyJS?

EasyJS was born from the need to simplify JavaScript syntax and make it more accessible to those new to programming, while reducing the boilerplate and complexity found in TypeScript. Built with clarity and ease-of-use in mind, EasyJS aims to:

1. Improve code readability with a concise, modern syntax.
2. Offer optional typing for better error checking.
3. Enable efficient compilation to native JavaScript.

Whether you're developing frontend interfaces, server-side logic, or full-stack applications, EasyJS strives to provide a seamless development experience.

## Installation

To get started, you have two primary installation options:

### 1. Download the Installer
Download and run the EasyJS installer from [GitHub](https://github.com/grupojvm/easyjs).

### 2. Clone the Repository
You can also clone the repository and build the binary yourself:

```bash
git clone https://github.com/grupojvm/easyjs.git
cd easyjs
cargo build --release
```

## Quickstart
### Using EasyJS
Once installed, you can interact with EasyJS in various ways:

- Compile EasyJS Code:

Compile `.ej` files to `.js` files to run on browser, servers, or other JavaScript enviroments.
```bash
easyjs compile myfile.ej
```

- Use the REPL:

Run the inline repl to easily test and write quick scripts.
```bash
easyjs repl
    ___       ___       ___       ___            ___       ___   
   /\  \     /\  \     /\  \     /\__\          /\  \     /\  \  
  /::\  \   /::\  \   /::\  \   |::L__L        _\:\  \   /::\  \ 
 /::\:\__\ /::\:\__\ /\:\:\__\  |:::\__\      /\/::\__\ /\:\:\__\
 \:\:\/  / \/\::/  / \:\:\/__/  /:;;/__/      \::/\/__/ \:\:\/__/
  \:\/  /    /:/  /   \::/  /   \/__/          \/__/     \::/  / 
   \/__/     \/__/     \/__/                              \/__/
EasyJS 0.4.2
>> // your code goes here.
```

- Using in browser:

```html
<head>
    <!-- include the easyjs compiler -->
    <script src="https://jordanmcastro.com/easyjs/cdn/compiler.min.js">
    </script>
</head>
<body>
    <script type="easjs"> 
        console.log("Hello world")
    </script>
</body>
```