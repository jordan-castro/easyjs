// ===== Builtin globals =====
/// global idx for heap memory
pub const GLOBAL_HEAP_IDX: u32 = 0;


// ===== Builtin functions =====
// =====      STRINGS      =====
/// IDX for __str_alloc
/// 
/// `str_length(0): int` Length of the string.
/// 
/// returns: `ptr: int` The pointer position
pub const ALLOCATE_STRING_IDX: u32 = 0;
/// IDX for __str_store_len
///
/// `ptr(0): int` The strings pointer
/// 
/// `length(1): int` The length of the string 
pub const STORE_STRING_LENGTH_IDX: u32 = 1;
/// IDX for __str_store_byte
/// 
/// `ptr(0): int` The string pointer
/// 
/// `position(1): int` The position to place the byte. This should also include the +4 length offset.
/// 
/// `byte(2): int` The char byte.
pub const STR_STORE_BYTE_IDX: u32 = 2;
/// IDX for __str_get_len
/// 
/// `ptr(0): int` The string pointer
/// 
/// returns: `length: int` The length bytes in ptr.
pub const STR_GET_LEN_IDX: u32 = 3;
/// IDX for __str_concat
/// 
/// `ptr_1(0): int` The pointer to the first string.
/// 
/// `ptr_2(1): int` The pointer to the second string.
/// 
/// returns: `ptr_3: int` The pointer to the new string.
pub const STR_CONCAT_IDX: u32 = 4;
/// IDX for __str_index
/// 
/// `ptr(0): int` The pointer to the string.
/// 
/// `index(1): int` The index
/// 
/// returns: `char_ptr: int` The pointer to the new created string.
pub const STR_INDEX_IDX: u32 = 5;
/// IDX for __str_char_code_at
/// 
/// `ptr(0): int` The pointer to the string.
/// 
/// `index(1): int` The index
/// 
/// returns: `char_code: int` The literal byte at that index.
pub const STR_CHAR_CODE_AT_IDX: u32 = 6;

// =====      Arrays      =====
/**
 * IDX for __arr_allocate
 * 
 * `capacity(0): int` the capacity to start with.
 * 
 * returns: `ptr: int` a pointer to the array.
 */
pub const ARR_ALLOCATE_IDX: u32 = 7;
/**
 * IDX for __arr_store_len
 * 
 * `ptr(0): int` The pointer of the array
 * `length(1): int` The length of the array
 */
pub const ARR_STORE_LENGTH_IDX: u32 = 8;
/**
 * IDX for __arr_store_cap
 * 
 * `ptr(0): int` The pointer of the array.
 * `capacity(1: int` The capicity to set.
 */
pub const ARR_STORE_CAPACITY_IDX: u32 = 9;
/**
 * IDX for __arr_get_len
 * 
 * `ptr(0): int` The pointer to the array.
 * 
 * returns: `length: int` The current length of the array
 */
pub const ARR_GET_LEN_IDX: u32 = 10;
/**
 * IDX for __arr_get_cap
 * 
 * `ptr(0): int` The pointer to the array.
 * 
 * returns: `capacity: int` The current capacity of the array.
 */
pub const ARR_GET_CAP_IDX: u32 = 11;
/**
 * IDX for __arr_reallocate
 * 
 * `ptr(0): int` The pointer to the array.
 * 
 * returns: `ptr:int` The new pointer to the array.
 */
pub const ARR_REALLOCATE_IDX: u32 = 12;

/**
 * IDX for __arr_push_int
 * 
 * `ptr(0): int` The pointer to the array.
 * `item(1): int` The integer to push to the array.
 * 
 * returns: `ptr: int` The pointer of the array (new or same).
 */
pub const ARR_PUSH_INT_IDX: u32 = 13;

/**
 * IDX for __arr_push_float
 * 
 * `ptr(0): int` The pointer to the array.
 * `item(1): float` The integer to push to the array.
 * 
 * returns: `ptr: int` The pointer of the array (new or same)
 */
pub const ARR_PUSH_FLOAT_IDX: u32 = 14;

/// IDX for __arr_push_string
/// 
/// `ptr(0): int` The pointer to the array.
/// `item(1): string` The pointer to the string.
/// 
/// returns: `ptr: int` The pointer of the array
pub const ARR_PUSH_STRING_IDX: u32 = 15;

/// IDX for __arr_push_array
/// 
/// `ptr(0): int` The pointer to the array.
/// `item:(1): array` The array to push to the array.
/// 
/// returns: `ptr: int` The pointer of the array (new or same)
pub const ARR_PUSH_ARRAY_IDX: u32 = 16;

/// IDX for __arr_get_item
/// 
/// `ptr(0): int` The pointer to the array.
/// `index(1): int` The index of the item
/// 
/// returns: `[type:int, int_val:int, float_val:float, string_val:string, array_val:array]` 
/// Returns the type, a integer value, a float value, a string value, and a array value.
pub const ARR_GET_ITEM_IDX: u32 = 17;

// pub const RE_ALLOCATE_ARRAY_IDX: u32 = 8;
// pub const ARR_GET_SIZE_IDX: u32 = 9;
// pub const DELETE_FROM_ARR_IDX: u32 = 10;
// pub const ADD_TO_ARR_IDX: u32 = 11;
// pub const ARR_INDEX_IDX: u32 = 12;

// ===== Builtin function names =====
// =====        STRINGS         =====
pub const ALLOCATE_STRING_NAME : &str = "__str_alloc";
pub const STORE_STRING_LENGTH_NAME :&str = "__str_store_len";
pub const STR_GET_LEN_NAME: &str = "__str_get_len";
pub const STR_STORE_BYTE_NAME: &str = "__str_store_byte";
pub const STR_CONCAT_NAME: &str = "__str_concat";
pub const STR_INDEX_NAME: &str = "__str_index";
pub const STR_CHAR_CODE_AT_NAME: &str = "__str_char_code_at";

// =====        ARRAYS          =====
pub const ARR_ALLOCATE_NAME : &str = "__arr_alloc";
pub const ARR_STORE_LENGTH_NAME: &str = "__arr_store_len";
pub const ARR_STORE_CAPACITY_NAME: &str = "__arr_store_cap";
pub const ARR_GET_LEN_NAME: &str = "__arr_get_len";
pub const ARR_GET_CAP_NAME: &str = "__arr_get_cap";
pub const ARR_REALLOCATE_NAME: &str = "__arr_reallocate";
pub const ARR_PUSH_INT_NAME: &str = "__arr_push_int";
pub const ARR_PUSH_FLOAT_NAME: &str = "__arr_push_float";
pub const ARR_PUSH_STRING_NAME: &str = "__arr_push_string";
pub const ARR_PUSH_ARRAY_NAME: &str = "__arr_push_array";
pub const ARR_GET_ITEM_NAME: &str = "__arr_get_item";