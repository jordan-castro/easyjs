# easyjsr
Default runtime for easyjs. Built on top of rquickjs[https://crates.io/crates/rquickjs/0.9.0]

## usage
To use with `easyjs` you can set the runtime to `easyjsr`.

```bash
easyjs repl --runtime easyjsr
> import 'std'
> @print('Hello World')
```

## For developers
This is a really easy to use runtime for FFI and embedding.

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