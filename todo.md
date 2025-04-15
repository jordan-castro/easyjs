# TODO

## Compiler
- v0.6.0 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - performance (vs JavaScript, TypeScript, Dart, WASM)
- v0.5.0 Better errors.
- v1.0.0 Add classes.
- v0.5.0 enums
- v0.5.0 type checker
- v0.6.0 add a local JS Engine
- v0.5.0 optional return statements in dynamic variable creations.
- v0.5.0 ~~Remove consts (use macro instead @const(name, value))~~
- v0.5.0 use_mod('lib', ...) macros(true or false), 
- v0.5.0 ~~Fix issue with javascript token in macros.~~
  - ~~also fix throwing issue.~~
- v1.0.0 dedicated DOM api

## Scripts
- v0.4.0 ~~add to path (also update path...)~~

## WASM
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
  - import functions from external.
  - loops
  - if statements
- v0.5.0
  - dicts
    - key, value
    - set, get by key
    - dot expression
  - classes
  - structs
- v0.6.0 
  - GC (Nah, handle your memory... it's not that hard...)
- v0.6.0 explore multi threaded wasm.
- v0.5.0 add shared memory support
- future version (when we have a team of developers) Switch to native wasm implementation...

## Repl

## Website
- v0.4.0 Links
  - Github repo
  - personal email
  - twitter
- v0.5.0 add examples 
  - native
  - fn variables
  - macros advanced.
