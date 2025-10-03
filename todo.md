# TODO

## Compiler
- v0.6 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - wasm compilation (vs AssemblyScript, Dart, Rust, Go, C/C++)
  - ease of use (vs TypeScript, Dart, CoffeScript)
- v0.5 type checker
- v0.5 Add classes
- v0.4 Finish adding all operators
  - <<
  - ^ 
- v0.4 allow non string keys in objects.
- ~~v0.4 update namespace to just use 1 '_' instead of 2~~
- v0.5.0 Small Interepteter for templates
- v0.5.0 Update tests
- v0.5.0 Add FFI options...
- ~~v0.5.0 Add fn(args) expression TO (args) => expression~~
- v0.5.0 Add macro(args) expression TO (args) => macro
- v0.5.0 Add extensions
  - macro extensions
  - non macro extensions
- v0.5.0 Doc comments for macros.
- Allow block statements in macros?
- Allow anonomous classes...

## Runtime
- ~~Use ejr instead.~~
- ~~v0.5.0 Update to use jsarg_make_list and jsarg_free_all instaed of creating *mut *mut~~
- ~~update text_encoder.js to text_encoder.ej~~
- Write Wasm Imp

## Commands
- v0.5.0 Remove the install command. Keep that with ezpkg.
  - ezpkg should be written mostly in native easyjs with Rust backed runtime features.
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
    - or
    - and
  - operators
    - ~~>~~
    - ~~<~~
    - ~~>=~~
    - ~~<=~~
    - ~~=~~
    - ~~+~~
    - ~~-~~
    - ~~*~~
    - ~~ \ ~~
    - ~~%~~
    - ~~+=~~
    - -=
    - *=
    - /=
    - |
    - &
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
- v0.4.0 Check () work
- v0.5.0 GC add a garbage collector OR a free method.
- v0.5.0 Allow for multiple return types in JS side
- v0.5.0 Dot methods

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
- v0.5.0 Start on a JS -> EJ lib conversion.

## Scripts
- Rewrite the scripts/ folder to pure easyjs
  - ~~load_std~~
  - build_releases
  - build
  - buildwasm
  - move_to_bin

## Tests
- add tests.