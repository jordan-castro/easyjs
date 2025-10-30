// 64 KB wasm page size
pub const PAGE_SIZE = 65536;

/// Heap memory
pub var heap_ptr : usize = 1024;

/// Type offset
pub var type_offset : usize = 4;

/// Nullptr
pub const nullptr = 0;

// Types
pub const IntType = 1;
pub const FloatType = 2;
pub const StringType = 3;
pub const ArrayType = 4;
pub const DictType = 5;
pub const StructType = 6;

/// Allocate new memory by `size`
pub fn malloc(size: usize) usize {
    const ptr = heap_ptr;
    heap_ptr += size;
    return ptr;
}

/// Set variable type
pub fn set_type(ptr: usize, var_type: usize) void {
    const type_ptr = @as(*usize, @ptrFromInt(ptr));
    type_ptr.* = var_type;
}

/// Get variable type
pub fn get_type(ptr: usize) usize {
    if (ptr == 0) {
        return nullptr;
    }

    const type_ptr = @as(*usize, @ptrFromInt(ptr));
    return type_ptr.*;
}

// /// TODO: Free allocated memory
// export fn __free(ptr: usize) void {

// }

