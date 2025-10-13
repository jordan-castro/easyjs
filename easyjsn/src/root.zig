//! The easyjs native runtime library.

const mem = @import("mem.zig");
const strings = @import("strings.zig");

export fn __malloc(size: usize) usize {
    return mem.malloc(size);
}

// ==================================== STRINGS

export fn __str_new(len: usize) strings.StringPtr {
    return strings.str_new(len);
}

export fn __str_len(ptr: strings.StringPtr) usize {
    return strings.str_len(ptr);
}

export fn __str_store_byte(ptr: strings.StringPtr, index: usize, value: u8) void {
    strings.str_store_byte(ptr, index, value);
}

export fn __str_store_bytes(ptr: strings.StringPtr, value: [*]const u8, len: usize) void {
    strings.str_store_bytes(ptr, value, len);
}

export fn __str_char_code_at(ptr: strings.StringPtr, index: usize) u8 {
    return strings.str_char_code_at(ptr, index);
}

export fn __str_char_at(ptr: strings.StringPtr, index:i32) strings.StringPtr {
    return strings.str_char_at(ptr, index);
}

export fn __str_to_upper(ptr: strings.StringPtr) strings.StringPtr {
    return strings.str_to_upper(ptr);
}

export fn __str_to_lower(ptr: strings.StringPtr) strings.StringPtr {
    return strings.str_to_lower(ptr);
}

export fn __str_concat(ptr1: strings.StringPtr, ptr2: strings.StringPtr) strings.StringPtr {
    return strings.str_concat(ptr1, ptr2);
}

export fn __str_slice(ptr: strings.StringPtr, start: usize, end: i32) strings.StringPtr {
    return strings.str_slice(ptr, start, end);
}

export fn __str_substr(ptr: strings.StringPtr, start: usize, end: i32) strings.StringPtr {
    return strings.str_substr(ptr, start, end);
}

// Strings end

// ========================================= ARRAYS
// Arrays end

// ========================================= Dictionaries
// Dictionaries end

// ========================================= STRUCTS
// Structs end
