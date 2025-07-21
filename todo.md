# TODO

## Compiler
- v0.6.0 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - wasm compilation (vs AssemblyScript, Dart, Rust, Go, C/C++)
  - ease of use (vs TypeScript, Dart, CoffeScript)
- v0.5.0 type checker
- v0.7.0 default function parameters.
- v0.7.0 Do named paramaters better. instead of wrapping with {} actually get the position and place it correctly! 
- v0.6.0 Add namespaces and just include with import 'file.ej' as file y ya.
- v0.5.0 Include the wasm binary with the CLI.
- v0.5.0 ~~Fix for loops in macros~~
- v0.5.0 ~~Fix javascript{} token first charater~~
- v0.5.0 Add classes
- v0.5.0 revamp structs.
- v0.4.x ~~add break and continue as tokens.~~
- v0.4.x Finish adding all operators
  - <<
  - ^ 
- v0.4.x ~~Update wasm usecase in transpiler~~
- v0.4.x ~~confirm imports of the same file work in the same project.~~
- v0.4.1 ~~importing native modules.~~
- v0.4.1 ~~Fix ?.~~ (removed it!)
- v0.4.2 better macro features.
  - v0.4.2 Optional paramaters
  - v0.4.2 N number of paramaters
- v0.4.2 compiler time macros

## Commands
- v0.4.5 Add a update command to the CLI
- v0.4.3 allow compilation via strings

## Scripts
- v0.4.0 ~~add to path (also update path...)~~

## WASM
- v0.4.x Additions
  - instruction generator
  - smart memory
    - strings
      - ~~add n strings together~~
      - ~~char_at~~
      - ~~char_code_at~~
      - slice
      - substring
      - ~~[]~~
      - to_upper_case
      - to_lower_case
      - ~~alloc~~
      - ~~negative indexing~~
      - use data for static strings.
    - arrays
      - push
      - append
      - [] object at position
      - remove
  - import functions from external.
  - loops
  - ~~if statements~~
  - operators
    - ~~>~~
    - ~~<~~
    - ~~>=~~
    - ~~<=~~
    - ~~=~~
    - ~~+~~
    - ~~-~~
    - ~~*~~
    - ~~\~~
    - ~~%~~
    - ~~+=~~
    - -=
    - *=
    - /=
- v0.4.0
  - dicts
    - key, value
    - set, get by key
    - dot expression
  - classes
  - structs
- v0.6.0 
  - GC
- v0.6.0 explore multi threaded wasm.
- v0.6.0 add shared memory support
- v0.4.x Generate WAT for debugging purposes.
- v0.4.0 Run a optomizer on the wasm byte code before transpiling.

## Website
- v0.4.0 Links
  - Github repo
  - personal email
  - twitter
- v0.5.0 add examples 
  - native
  - fn variables
  - macros advanced.

## Docs
- Write better documentation

## Github
- start realeasing versions.

## lib
- v0.4.x shell commands

# Notes
- ~~self in async struct methods.~~
- ~~Def gotta work on struct parsing.~~
- ~~Parse doc comments better.~~
- ~~fix the double ';' issue~~
- we definitely want to add class support
- gotta fix "in" keyword
    - chunk.id in chunks => chunk.chunks.includes(id) 
    - chunk.id in chunks => chunks.includes(chunk.id)
