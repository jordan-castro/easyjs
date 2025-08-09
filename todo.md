# TODO

## Compiler
- v0.6.0 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - wasm compilation (vs AssemblyScript, Dart, Rust, Go, C/C++)
  - ease of use (vs TypeScript, Dart, CoffeScript)
- v0.5.0 type checker
- v0.7.0 ~~default function parameters.~~
- v0.7.0 ~~Do named paramaters better. instead of wrapping with {} actually get the position and place it correctly! ~~
- v0.5.0 Include the wasm binary with the CLI.
- v0.5.0 Add classes
- v0.4.x Finish adding all operators
  - <<
  - ^ 
- v0.4.5 ~~better macro features~~.
  - v0.4.5 ~~N number of paramaters (think ...args)~~
  - v0.4.5 ~~default paramaters in macros~~
- v0.4.5 Check vaiable scoping from imported files
- ~~v0.6.0 Add namespaces and just include with import 'file.ej' as file y ya.~~
- v0.5.0 add templates
  - classes via template
  - HTML via templates
  - compile time with `easyjsr`
- v0.4.0 allow non string keys in objects.
- v0.4.0 Allow "as _" in modules.

## Commands
- v0.4.5 Add a update command to the CLI
- v0.4.5 allow compilation via strings

## Native (WASM)
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
- v0.4.0 Allow for calling functions from client easyjs in native. i.e. easyjs function/variable/struct used within native block.
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

## lib
- v0.4.x shell commands
- v0.4.x Universal std lib:
  - io
  - sys

## Scripts
- Rewrite the scripts/ folder to pure easyjs
  - ~~load_std~~
  - build_releases
  - build
  - buildwasm
  - move_to_bin