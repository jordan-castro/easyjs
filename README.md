# easyjs
easyjs is a general purpose programming language that compiles to JS and WASM. All within the same program.
JavaScript has a huge ecosystem and runs natively in the browser.
JS alongside with WASM, macros, types, and a standard library becomes really powerful.

> [!WARNING]  
> This language is still in development we are currently on v0.5.0

## Install
To install you have a few options.

### Download
Download from releases.

### GIT
Clone this Git repo and run `cargo build --release` to build the binary.

## How to use
You have many different options to use. 

### Compile
You can compile easyjs to a js file to run on the browser, server, etc.
```bash
easyjs file.ej file.js
```

### Script tag
You can use a `<script type="easyjs">` tag in the browser to inline the easyjs. <-- This requires the easyjs wasm runtime.

You can use a `<script src="source.js">` tag in the browser.

### Repl
easyjs provides a REPL. Use it by running `easyjs` in your terminal.
```bash
easyjs
> // your code goes here.
```

You can use any of the following runtimes
- node
- deno
- pxs

`pxs` is the default runtime. To learn more about [pxs](https://github.com/jordan-castro/pixelscript).

### Online
You can also go to the [easyjs website](https://jordanmcastro.com/easyjs)

### Run file
Run a file directly with:
`easyjs file.ej`

## Key features
- Modern syntax.
- Optional typing.
- Wasm/native compilation targets.
- Text injection macros.

## Example
Imagine you have a easyjs file like so:

```js
// ================ Variables ================
// To declare immutable variables
hello := "World"
// Immutable variables are typed by default.

// To declare mutable typed variables
world = "Hello"

// To declare a dynamic typed variable
number: dyn =  0
number = 0.5 // won't crash!

// ================ Functions ================
sum_numbers := fn(numbers: []int) int {
    sum = 0
    for n in numbers {
        sum += n
    }
    sum
}

print(sum_numbers([1,2,3])) // 6

// Importing the standard library
types := import("std.types")
object := import("std.object")

// To do js style imports you need to use a macro
jsimport(module, { method })
// Which compiles to `import { method } from 'module';`
// But this won't work unless at the top of the file.

// Dfine `enum` as a macro function which gets expanded at compile time.
@macro
enum := fn(...idents: []types.Ident) {
    compfn{ // compfn runs logic at compile time.
        res = class {
            map : Map[string, dyn]
            // Overloaded on the `.` operator.
            __dot__ := fn(this, key) {
                if key in map {
                    return this.map[key]
                } else {
                    return null
                }
            }
        }

        for id in idents {
            res.map.insert(id, id.to_string())
        }

        object.freeze(res)
        
        res
    }
}
// Use it here.
Results := enum(
    Name,
    Age,
    Date
)


// ================ Objects ================
// Defines a Prototype
Person := class {
    name: string
    age: int

    // Custom constructor
    new := fn(name:string, age:int) Person {
        Person(name, age + 1)
    }

    // Magic '+' overloader
    __add__ := fn(this, other:Person) Person {
        Person.new(this.name + other.name, this.age + other.age)
    }

    // You can have enums within a class too. Access it via `Person.job_types.*`
    job_types := enum(Programmer, Engineer, Deveoper)

    // Assign it to a variable.
    // Within the class you don't need to add `Person`.
    job_type = job_types.Programmer
}

// Make a new instace of the class
person := Person.new("Jordan", 24)
print(person.name)
print(person.age)

// ================ Macros ================
@macro
println := fn(name) {
    print(name, "\n")
}

println(person)

// =============== WASM ===============
@wasm 
wasm_sum := fn(nums: []int) int {
    sum = 0
    for n in nums {
        sum += n
    }
    sum
}

print(wasm_sum([10, 12, 13]))

@wasm
wasm_Person := class {
    name: string,
    age: int,

    new := fn(name: string, age: int) wasm_Person {
        wasm_Person(name, age + 1)
    }

    // Magic '-' overloader
    __sub__ := fn(this, other:wasm_Person) wasm_Person {
        wasm_Person.new(this.name + other.name, this.age + other.age)
    }
}

// Create wasm class
wperson := wasm_Person.new("Evelyn", 21)
print(wperson.name)
print(wperson.age)

// wasm supports (int, float, string, array, map, class)
@wasm
w_float := 1.0
@wasm
w_string := "hello"
@wasm
w_list := [0,1,2]
```

Yes it is that easy!


## STD
We all know that JS does not have a standard library. The great thing about easyjs is that... it does!

### Browser 
Easyjs comes packed with browser specific apis.

```js
// Import browser apis
dom := import("std.browser.document")
el := import("std.browser.element")
css := import("std.browser.css")
dom.add(
    el.H1(
        "Hello World!", 
        css.Style(
            color = css.WHITE,
            text_align = css.TextAlign.Start
        )
    ),
    el.Br(),
    el.P("Yep it is that easy!")
)
```

### Bundled
Also provided are a suite of vendored libraries that are bundled with easyjs.
```js
// Import the fs api.
fs := import("std.fs")

// Read a files contents
print(fs.read_file("path/to/file.json"))
```

On a browser this will still work because of compile time pragmas.
```js
#if BROWSER
browser := import("std.browser")
#else
native := import("std.native")
#endif

// browser implementation fo `fs.read_file`
read_file := fn(path:string) string {
    #if BROWSER
        return await browser.fetch(browser.window.location + path)
    #else
        return await native.read_file(path)  
    #endif
}

// Yes, IT REALLY IS THAT EASY!
```

### C FFI
In easyjs it is also possible to interop with C libraries, that is how most of `std` is implemented.

Take a simple c libary.

```c
int add(int a, int b) {
    return a + b;
}
```
To use it from easyjs...
```js
compfn{ // handle it at compile time.
    ffi := import("std.ffi")
    module := ffi.include(["math.h"]).compile()
}
// now call
print(module.add(1,2)) // 3
```
This works in the browser too, only that it must be compiled AOT.

## Used in
- Jorlyn Movie, a webapp for recording and editing videos.