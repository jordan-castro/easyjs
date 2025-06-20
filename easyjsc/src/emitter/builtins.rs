// ===== Builtin globals =====
/// global idx for string memory
pub const GLOBAL_STRING_IDX: u32 = 0;


// ===== Builtin functions =====
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

// ===== Builtin function names =====
pub const ALLOCATE_STRING_NAME : &str = "__str_alloc";
pub const STORE_STRING_LENGTH_NAME :&str = "__str_store_len";
pub const STR_GET_LEN_NAME: &str = "__str_get_len";
pub const STR_STORE_BYTE_NAME: &str = "__str_store_byte";
pub const STR_CONCAT_NAME: &str = "__str_concat";
pub const STR_INDEX_NAME: &str = "__str_index";
pub const STR_CHAR_CODE_AT_NAME: &str = "__str_char_code_at";