const mem = @import("mem.zig");

/// String offset
pub const string_offset = 4;
/// Tthe StringPtr type
pub const StringPtr = usize;

/// Store a new string.
/// 
///- `len` The initial length of the string.
pub fn str_new(len: usize) StringPtr {
    const ptr = mem.malloc(mem.type_offset + string_offset + len);
    mem.set_type(ptr, mem.StringType);
    const len_ptr = @as(*usize, @ptrFromInt(ptr + mem.type_offset + string_offset));
    len_ptr.* = len;
    return ptr;
}


/// Read the length of a string from ptr.
pub fn str_len(ptr: StringPtr) usize {
    if (ptr == mem.nullptr or mem.get_type(ptr) != mem.StringType) {
        return mem.nullptr;
    } 
    const len_ptr = @as(*usize, @ptrFromInt(ptr));
    return len_ptr.*;
}

/// Store a byte in a string
/// 
/// - `index` The byte position. Must be less than the current length
/// - `value` The byte itself.
pub fn str_store_byte(ptr: StringPtr, index: usize, value: u8) void {
    if (mem.get_type(ptr) != mem.StringType) {
        return;
    }

    // Check length
    const len = str_len(ptr);
    if (index >= len) {
        return;
    }

    const addr = @as(*u8, @ptrFromInt(ptr + mem.type_offset + string_offset + index));
    addr.* = value;
}

/// Store a sequence of bytes
/// 
/// - `value` The bytes
/// - `length` The length of the bytes.
pub fn str_store_bytes(ptr: StringPtr, value: [*]const u8, len: usize) void {
    if (mem.get_type(ptr) != mem.StringType) {
        return;
    }
    // Check length
    if (len >= str_len(ptr)) {
        return;
    }

    // Loop dayo!
    for (0..len) |i| {
        str_store_byte(ptr, i, value[i]);
    }
}

/// Get a character code of a string at `index`.
/// 
/// `index` must be less than or equal to length
pub fn str_char_code_at(ptr: StringPtr, index: usize) u8 {
    if (mem.get_type(ptr) != mem.StringType or index >= str_len(ptr)) {
        return 0;
    }

    const value = @as(*u8, @ptrFromInt(ptr + mem.type_offset + string_offset + index));
    return value.*;
}

/// Get a character of a string at `index`.
/// 
/// `index` must be less than or qual to length. Returns a string `ptr`.
pub fn str_char_at(ptr: StringPtr, index:i32) StringPtr {
    if (mem.get_type(ptr) != mem.StringType) {
        return mem.nullptr;
    }

    const len = str_len(ptr);
    if (index >= len) {
        // Invalid ptr.
        return mem.nullptr;
    }

    // Allow for negative indexing.
    var r_index = index;
    const len_i32: i32 = @intCast(len);
    if (index < 0) {
        r_index = len_i32 + index;
    }

    const char_code = str_char_code_at(ptr, @intCast(r_index));
    // Convert to string
    const str = str_new(1);
    str_store_byte(str, 0, char_code);
    return str;
}

/// Convert a string to capital.
/// 
/// Allocates a new string.
pub fn str_to_upper(ptr: StringPtr) StringPtr {
    if (mem.get_type(ptr) != mem.StringType) {
        return mem.nullptr;
    }

    const len = str_len(ptr);
    const nptr = str_new(len);

    for (0..len) |i| {
        const byte = str_char_code_at(ptr, i);
        if (byte >= 'a' and byte <= 'z') {
            str_store_byte(nptr, i, byte - 32);
        } else {
            str_store_byte(nptr, i, byte);
        }
    }

    return nptr;
}

/// Convert a string to lowercase
/// 
/// Allocates a new string.
pub fn str_to_lower(ptr: StringPtr) StringPtr {
    if (mem.get_type(ptr) != mem.StringType) {
        return mem.nullptr;
    }

    const len = str_len(ptr);
    const nptr = str_new(len);

    for (0..len) |i| {
        const byte = str_char_code_at(ptr, i);
        if (byte >= 'A' and byte <= 'Z') {
            str_store_byte(nptr, i, byte + 32);
        } else {
            str_store_byte(nptr, i, byte);
        }
    }

    return nptr;
}

/// Concatonate strings together
/// 
/// `str1` and `str2` must both be strings.
pub fn str_concat(str1: StringPtr, str2: StringPtr) StringPtr {
    if (mem.get_type(str1) != mem.StringType or mem.get_type(str2) != mem.StringType) {
        return mem.nullptr;
    }

    const len1 = str_len(str1);
    const len2 = str_len(str2);

    const nlen = len1 + len2;
    const nptr = str_new(nlen);

    for (0..nlen) |i| {
        var byte:*u8 = undefined;
        if (i < len1) {
            byte = @as(*u8, @ptrFromInt(str1 + mem.type_offset + string_offset + i));
        } else {
            byte = @as(*u8, @ptrFromInt(str2 + mem.type_offset + string_offset + (i - len1)));
        }
        str_store_byte(nptr, i, byte.*);
    }

    return nptr;
}

/// Get a slice from a string.
/// 
/// Allocates a new string
pub fn str_slice(ptr:StringPtr, start:usize, stop:i32) StringPtr {
    if (mem.get_type(ptr) != mem.StringType) {
        return mem.nullptr;
    }

    const len = str_len(ptr);

    // Get the real end of the slice
    var r_end = stop;
    if (stop < 0) {
        r_end = @as(i32, @intCast(len)) + stop;
    }
    
    var r_start:i32 = @intCast(start);
    if (r_start >= len) {
        r_start = @as(i32, @intCast(len)) - 1;
    } 
    // Ensure that r_end is greater than start
    if (r_end < r_start) {
        // Swap
        const s = r_start;
        const e = r_end;
        r_start = e;
        r_end = s;
    }

    const nptr = str_new(@intCast(r_end - r_start));
    
    for (@as(usize, @intCast(r_start))..@as(usize, @intCast(r_end))) |i| {
        const nindex = i - @as(usize, @intCast(r_start));
        const byte = @as(*u8, @ptrFromInt(nptr + mem.type_offset + string_offset + i));
        str_store_byte(ptr, nindex, byte.*);
    }

    return nptr;
}

/// Get a substring frmo a string.
/// 
/// Allocates a new string
pub fn str_substr(ptr:StringPtr, start:usize, stop:i32) StringPtr {
    if (mem.get_type(ptr) != mem.StringType) {
        return mem.nullptr;
    }

    const len = str_len(ptr);

    var r_start:i32 = @intCast(start);
    if (r_start >= len) {
        r_start = @as(i32, @intCast(len)) - 1;
    }
    var r_end = stop;
    if (r_end < 0) {
        r_end = 0;
    }
    if (r_end >= len) {
        r_end = @as(i32, @intCast(len)) - 1;
    }

    if (r_start > r_end) {
        const s = r_start;
        const e = r_end;

        r_end = s;
        r_start = e;
    }

    return str_slice(ptr, @intCast(r_start), r_end);
}