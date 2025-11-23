# TODO

## Compiler
- v0.6 Add benchmarks for:
  - compilation (vs TypeScript, Dart, Nim, other to JS options)
  - wasm compilation (vs AssemblyScript, Dart, Rust, Go, C/C++)
  - ease of use (vs TypeScript, Dart, CoffeScript)
- v0.5 type checker
  - basic types
    - int
    - float
    - string
    - dyn
    - array
- ~~v0.5 Add classes~~
- v0.4 Finish adding all operators
  - <<
  - ^ 
- ~~v0.4 allow non string keys in objects.~~
- ~~v0.4 update namespace to just use 1 '_' instead of 2~~
- v0.5.0 Add FFI options...
- ~~v0.5.0 Add fn(args) expression TO (args) => expression~~
- ~~v0.5.0 Add macro(args) expression TO (args) => macro~~
- v0.7.0 Add extensions
  - macro extensions
  - non macro extensions
  - extension on String {
    fn capFirst(self):String {
      captialize!(self,)[0] + self.substr(1)
    }
  }
  - x = "test".capFirst()
- v0.5.0 Doc comments for macros.
- Allow block statements in macros?
- v0.5.0 Optomize structs
- v0.6.0 Add proxy helpers
- Optomize compiler.
  - `Box<Expression>` not needed in Statement ast
  - reference rather than own/clone in transpiler.
- v0.6.0 Possibly allow anonomous functions and classes?
- v0.5.0 Remove := option. Always infer types (native only)
- ~~v0.5.0 Remove '@' symbol on macros~~
- ~~v0.5.0 Add hygenic macros~~
- v0.5.0 Support for calling private class functions and variables.
- ~~v0.5.0 Allow macros to except expressions~~
- ~~v0.5.0 Disallow macros to accept kwargs~~
- ~~v0.5.0 Remove kwargs support. JS alrady supports it~~
- v0.5.0 Fix to call super() always in classes

## Runtime
- Write Wasm Imp
  - strings
    - https://www.w3schools.com/js/js_string_methods.asp
    - tests
- ~~Fix io~~
- Fix build script to build bindings JIT.

## Commands
- ~~v0.5.0 Remove the install command. Keep that with ezpkg.~~
- v0.4.6 Add a update command to the CLI, this will install the most recent version from Github.
- v0.4.6 allow compilation via strings
- ~~v0.4.6 Make the default `easyjs` command default to repl.~~
- ~~v0.5.0 When calling easyjs file.ej (assume running)~~
- ~~v0.4.6 When calling easyjs file.ej file.js (assume compiling)~~

## Native (WASM)
- v0.4.6 WASM runtime in native.
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
- ~~v0.5.0 Actual globals~~
- ~~v0.5.0 Dynamic wasm core~~
- ~~v0.5.0 None (Void) methods~~
- v0.5.0 Add a main() function for Global initializtions.

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
- TODO
- Document how to build easyjs compiler

## lib
- v0.5.0 Start on a JS -> EJ lib conversion.
- ~~v0.5.0 Update const to be @const(expr) => const expr;~~

## Scripts
- Rewrite the scripts/ folder to pure easyjs
  - ~~load_std~~
  - build_releases
  - build
  - buildwasm
  - move_to_bin

## Package manager (ezpkg)
- v0.6.0 Try to use as much native as possible (if possible)
- v0.6.0 Backend with ejr (rust based)
- v0.6.0 For config files:
  - JSON (Is the easiest and most well-known...)
  - TOML (Is smaller, easier once learned...)
  - easyjs (Write code? Just in a VERY sandboxed enviroment) YUH!
- v0.6.0 Option to compile directly to a .wasm file and skip the .js file

## Tests
- add tests.