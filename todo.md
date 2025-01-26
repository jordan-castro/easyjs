# TODO

## Compiler
- v0.3.0 add a LSP and text highlighting
- v0.3.0 implement actual decorators.
- v0.3.0 Introduce variable scoping
- v0.4.0 add a minifier option.
- v0.5.0 decopouling vairable decleration <!-- This can only be done when types are added -->
  - var x,y = array // [100,200]
  - var x,y = dictionary // {x: 10, y: 20}
  
- ~~v0.3.0 add switch case~~
- ~~v0.3.0 add a way to determine if running in browser or nodejs/deno/bun~~
- ~~v0.3.0 use pub instead of export~~
- ~~v0.3.0 add a async block~~
- v0.4.0 Support UNICODE 
- ~~v0.3.0 add a default _ placeholder~~
- ~~v0.3.0 Support Doc Comments~~
- ~~v0.3.0 add support for base:file~~
- v0.3.0 Support struct without new when coming from modules.
- v0.3.1 Fix builtins.int_range logic.
- v0.3.0 Add a new way to handle .then logic.
- v0.4.0 add types (using boa)
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
- v0.3.0 core
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
- v0.4.0 compile wasm functions AOT or JIT or don't compile and just interpret at the Rust Level.

## Repl

## Docs

## Extension

## Build Tool
- v0.5.0 Add a project generator (this should be easy...)
- v0.5.0 Add npm package support.
  - Must include : (compiler, wasmer) as global functions (easyjs.compile, easyjs.wasmer.compile/run)


## Website
- v0.3.0 Links
  - Github repo
  - personal email
  - twitter
  - discord?