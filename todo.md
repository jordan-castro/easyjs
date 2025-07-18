# TODO

## Compiler
- v0.6.0 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - wasm compilation (vs AssemblyScript, Dart, Rust, Go, C/C++)
  - ease of use (vs TypeScript, Dart, CoffeScript)
- v0.5.0 type checker
- v0.7.0 default function parameters.
- v0.7.0 Do named paramaters better. instead of wrapping with {} actually get the position and place it correctly! 
- v0.6.0 Add namespaces and just include with import 'file.ej' y ya. No as.
- v0.5.0 Include the wasm binary with the CLI.
- v0.5.0 Fix for loops in macros
- v0.5.0 Fix javascript{} token first charater
- v0.5.0 Add classes
- v0.5.0 revamp structs.
- v0.4.x add break and continue as tokens.
- v0.4.x Finish adding all operators
  - <<
  - ^ 
- v0.4.x revamp IIFEs. Using get_return logic
- v0.4.x Update wasm usecase in transpiler
  - I'm trying to do so that you need to do:
    - // pseduo code
    - module = await load_wasm(__native) 
    - module.fun_name(arg1, arg2, ...etc)
    - I definietely need to add a TypeChecker to the transpiler. I might need to refactor some parts of it.
    - But I wonder...
    - What if we treated the transpiler as INSTRUCTION based?
    - that would be crazy right?
    - hmmm, that might be really advanced actually
- v0.4.x confirm imports of the same file work in the same project.
- v0.4.1 Fix ?.

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