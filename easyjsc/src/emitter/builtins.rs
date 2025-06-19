// ===== Builtin globals =====
/// global idx for string memory
pub const GLOBAL_STRING_IDX: u32 = 0;


// ===== Builtin functions =====
/// IDX for __str_alloc
pub const ALLOCATE_STRING_IDX: u32 = 0;
/// IDX for __str_store_len
pub const STORE_STRING_LENGTH_IDX: u32 = 1;
/// IDX for __str_store_byte
pub const STR_STORE_BYTE_IDX: u32 = 2;
/// IDX for __str_get_len
pub const STR_GET_LEN_IDX: u32 = 3;

// ===== Builtin function names =====
pub const ALLOCATE_STRING_NAME : &str = "__str_alloc";
pub const STORE_STRING_LENGTH_NAME :&str = "__str_store_len";
pub const STR_GET_LEN_NAME: &str = "__str_get_len";
pub const STR_STORE_BYTE_NAME: &str = "__str_store_byte";