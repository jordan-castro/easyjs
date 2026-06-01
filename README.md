# easyjs
easyjs is a general purpose programming language that compiles to JS, WASM, and Native targets. All within the same program.
JavaScript has a huge ecosystem and runs natively in the browser.
JS alongside with WASM, macros, and types becomes really powerful.

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

// To declare mutable variables
world = "Hello"

// Type variable mutable
number : int = 0
decimal : float = 0.5
// Immutable variables are typed by default.

// ================ Functions ================
sum_numbers := fn(numbers: []int) :: int {
    sum = 0
    for n in numbers {
        sum += n
    }
    sum
}

print(sum_numbers([1,2,3])); // 6

// Import types
types := import("std.types")
js := import("std.js")

@macro
enum := fn(...idents: []types.Ident) :: js.Object {
    object : js.Object = js.object()
    for i in idents {
        object[i.to_string()] = i.to_string()
    }
    object
}

javascript{
    let Results = (function() {
        let object = {};
        for (let i of [__ej_Ident("Name"), __ej_Ident("Age"), __ej_Ident("Date")]) {
            object[i.to_string()] = i.to_string();
        }
        return object;
    })();
}

// ================ Objects ================
// Defines a Prototype
Person := class {
    name: string
    age: int

    // Custom constructor
    new := fn(name:string, age:int) :: Person {
        Person(name, age + 1)
    }

    // Magic '+' overloader
    __add__ := fn(this, other:Person) :: Person {
        Person.new(this.name + other.name, this.age + other.age)
    }

    // You can have enums within a class too. Access it via `Person.job_types.*`
    job_types := enum(Programmer, Engineer, Deveoper)

    // Assign it to a variable.
    // Within the class you don't need to add `Person`.
    job_type : job_types = job_types.Programmer
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
wasm_sum := fn(nums: []int) :: int {
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

    new := fn(name: string, age: int) :: wasm_Person {
        p = wasm_Person(name, age)
        p
    }

    // Magic '-' overloader
    __sub__ := fn(this, other:wasm_Person) :: wasm_Person {
        wasm_Person.new(this.name + other.name, this.age + other.age)
    }
}

// Create wasm class
wperson := wasm_Person.new("Evelyn", 21)
print(wperson.name)
print(wperson.age)

// wasm supports (int, float, string, array, class)
@wasm
w_float := 1.0
@wasm
w_string := "hello"
@wasm
w_list := [0,1,2]

// =============== Native ===============
@native
native_sum := fn(nums: []int) :: int {
    sum = 0
    for n in nums {
        sum += n
    }
    sum
}
```

Yes it is that easy!

> [!WARNING]  
> Native compilation won't work in browser targets.
> Native FFI is only supported for nodejs, bun, deno, and yoyo.
> Currently only supporting function callbacks.
> For other runtimes see [How to implement easyjs ffi in custom runtime](https://) 
