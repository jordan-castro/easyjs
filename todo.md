# TODO

## Compiler
- v0.6.0 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - wasm compilation (vs TypeScript, Dart, Nim, other to WASM options)
  - ease of use (vs TypeScript, Dart, etc)
- v0.5.0 type checker
- v0.7.0 default function parameters.
- v0.7.0 Do named paramaters better. instead of wrapping with {} actually get the position and place it correctly! 
- v0.6.0 Add namespaces and just include with import 'file.ej' y ya. No as.
- v0.5.0 Include the wasm binary with the CLI.

## Commands
- v0.4.5 Add a update command to the CLI

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

## Website
- v0.4.0 Links
  - Github repo
  - personal email
  - twitter
- v0.5.0 add examples 
  - native
  - fn variables
  - macros advanced.
