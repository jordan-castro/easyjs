# TODO

## Compiler
- v0.5.0 add a LSP and text highlighting
- v0.4.0 ~~add a minifier option.~~
- v0.5.0 decopouling vairable decleration 
  - var x,y = array // [100,200]
  - var x,y = dictionary // {x: 10, y: 20}
- v0.4.0 Rework imports.  
- v0.4.0 ~~Support UNICODE~~
- v0.4.0 ~~Fix builtins.int_range logic.~~ (I think I'll just remove it. Only using it with for loops.)
- v0.4.0 ~~add types~~
- v0.4.0 ~~add ternirary statements (using ? :)~~
- v0.6.0 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - performance (vs JavaScript, TypeScript, Dart, WASM)
- v0.5.0 Better errors.
- v0.5.0 Make the change to use &str as much as possible
- v0.5.0 Add classes.
- v0.5.0 ~~Add rust like decleratoin var some = {};~~
- v0.5.0 ~~Allow nameless JS objects like: {x} would be the same as {x: x}~~
- v0.5.0 enums
- v0.5.0 type checker
- v0.6.0 add a local JS Engine
- v0.5.0 optional return statements in dynamic variable creations.

## Scripts
- v0.4.0 ~~add to path (also update path...)~~

## WASM
- v0.4.0 use ASC
- v0.4.0 ~~wasmer functions~~
- v0.4.0 ~~compile wasm funct.ions AOT or JIT or don't compile and just interpret at the Rust Level.~~ AOT
- v0.4.0 Additions
  - smart memory
    - strings
      - add 2 strings together
      - [] char at position
      - toUpper 
      - toLower
    - arrays
      - push
      - append
      - [] object at position
      - remove
    - dicts
      - key, value
      - set, get by key
      - dot expression
    - classes
    - structs
    - GC
  - ~~global variables~~
  - import functions from external.
  - ~~call other functions~~
  - loops
  - if statements
  - ~~math statements with consts~~
- v0.6.0 explore multi threaded wasm.
- v0.5.0 add shared memory support
- future version (when we have a team of developers) Switch to native wasm implementation...

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