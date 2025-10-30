# easyjsr
Default runtime for easyjs. Rust wrapper of [ejr](https://github.com/jordan-castro/ejr).

## usage
To use with `easyjs` you can set the runtime to `easyjsr`. It is also the default runtime.

```bash
easyjs repl --runtime easyjsr
> import 'std'
> @print('Hello World')
```

## For developers
This is a really easy to use runtime for embedding in rust projects. Important thing to note is that it does not currently support MSVC builds. 
Only GNU/Clang. So if using windows make sure to install the correct build system.
```bash
rustup target add x86_64-pc-windows-gnu
# And build with
cargo build --target x86_64-pc-windows-gnu
```

### Evaluating JS
```rust
let result = ejr.eval("1 + 1");
println!("{}", ejr.val_to_string(result)); // 2
```

### Calling specific functions
```rust
let script = r##"
    function say_hello_to(name) {
        console.log('Hello', name);
    }
"##;

let result = ejr.call("say_hello_to", vec!["Jordan"]); // Hello Jordan
```

### Creating callables
```rust
fn ___print(msg: String) {
    println!("{msg}");
}

ejr.register_callback("___print", ___print);

// Lambdas
ejr.register_callback("___log", |msg: String| {
    println!("{msg}");
});
```

### Compiling JS into exe
This is mostly for libraries, you usually won't use this in an app.
```rust
compile_js_code(code);

// Or as a module
compile_js_code_as_module(code);
```

## Use case
The use case of another JS runtime is specifically to be a high level wrapper for easy FFI use in:
- Kazoku
- easyjs
- Going Up

All projects that use easyjs as scripting.