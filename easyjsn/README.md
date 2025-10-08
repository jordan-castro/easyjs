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
native {
    /// Base structore for all memory types
    struct BaseMemrory[
        type: int
    ] {
        fn __set_type(self, type:int) {
            self.type = type
        }
    }

    /// A String is a wrapper around a native string.
    struct String[
        ptr: int
    ] with BaseMemory {
        fn __new__(ptr: int):String {
            String(ptr)
        }

        fn len(self):int {
            __str_len(self.ptr)
        }

        fn __add__(self, other: string|String):String {
            if other.type == self.type {
                String(__str_concat(self.ptr, other.ptr))
            } else {
                String(__str_concat(self.ptr, other))
            }
        }
    }
}
```