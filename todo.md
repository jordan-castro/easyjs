# TODO

## Compiler
- v0.5.0 add a LSP and text highlighting
- v0.4.0 add a minifier option.
- v0.5.0 decopouling vairable decleration 
  - var x,y = array // [100,200]
  - var x,y = dictionary // {x: 10, y: 20}
- v0.4.0 Rework imports.  
- v0.4.0 Support UNICODE 
- v0.4.0 Fix builtins.int_range logic. (I think I'll just remove it. Only using it with for loops.)
- v0.4.0 ~~add types~~
- v0.4.0 add ternirary statements (using ? :)
- v0.4.0 add if expressions (compiled into ternirary statements)
- v0.4.0 add advanced macros 
  - v0.4.0 macros that write easyJS.
  - v0.4.0 macros that run WASM.
- v0.4/5/6.0 add a fn {} block for creating modules. Or a mod {}?
- v0.6.0 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - performance (vs JavaScript, TypeScript, Dart, WASM)

## EasyJS
- v0.5.0 core
    - http
    - ~~json~~
    - math
    - random
    - ui
    - wasm
    - pyscript

## Macros

## Scripts
- v0.4.0 add to path (also update path...)

## WASM
- v0.4.0 wasmer functions
- v0.4.0 compile wasm funct.ions AOT or JIT or don't compile and just interpret at the Rust Level.

## Repl

## Docs

## Extension

## Build Tool
- v0.5.0 Add a project generator (this should be easy...)
- v0.5.0 Add npm package support.
  - Must include : (compiler, wasmer) as global functions (easyjs.compile, easyjs.wasmer.compile/run)


## Website
- v0.4.0 Links
  - Github repo
  - personal email
  - twitter
  - discord?
- v0.4.0 Add codemirror to web editor