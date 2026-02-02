# easyjs Version History.
easyjs version history starts from v0.4.3 onward.

## v0.4.3
- Add native block that compiles into WASM. Supported easyjs is:
    - math (+-*/)
    - types:
        - i32
        - i64
        - f32
        - f64
    - call expressions

## v0.4.4
- Compiler
    - Namespaces as (import 'path/to/file.ej' or import 'std' and the alias will be the file name.) If you want to place it in global scope use 'as _'
    - Native support
        - Strings
        - Ptr magic
        - All in easyjs
    - Expressions

- Runtime
    - Deprecated EJR, using PixelScript instead with JS backend


