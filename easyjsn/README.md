# easyjsn (easyjs native)
Easyjs native (wasm) runtime. In easyjs there is a optinal performance boost and low level code operations you can unlock by wrapping your code in a `native` block like:

```easyjs
native {
    pub fn add(x:int, y:int):int {
        x + y
    }
}
```
For simple operations it uses raw wasm instructions. But for implementing complex memory types like:
- Strings
- Arrays
- Dictionaries
- Structs

It's easier to write the runtime in a higher level language like zig. Zig was chosen because:
- First class wasm support.
- Small build sizes.
- Low level memory control

An example for strings:
```easyjs
import "nat/std.ej"
native {
    pub fn hello_world() {
        nat.std.print!("Hello World!")
        // What this really does is...
        ptr = __str_new(12)
        __str_store_byte(ptr, 0x...)
        __str_store_byte(ptr, 0x...)
        __str_store_byte(ptr, 0x...)
        // 12 times
        __easyjsnative_print_wrapper(ptr)
    }
}
```

## Build system
I don't use the zig build system. I think Python is better for this specific project since it's not using any 3rd party libraries or STD.
We just use a simple Python script, `build.py`.