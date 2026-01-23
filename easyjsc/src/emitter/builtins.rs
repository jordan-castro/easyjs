// ===== Builtin names =====
// =====        nat/mem.ej          =====
pub const SET_PTR_NAME: &str = "__set_ptr__";
pub const GET_PTR_NAME: &str = "__get_ptr__";
pub const MALLOC_NAME: &str = "__malloc__";
pub const SET_TYPE_NAME: &str = "__set_type__";
pub const GET_TYPE_NAME: &str = "__get_type__";


// =====        nat/strings.ej         =====
pub const ALLOCATE_STRING_NAME : &str = "__str_alloc__";
pub const STORE_STRING_LENGTH_NAME :&str = "__str_store_len__";
pub const STR_GET_LEN_NAME: &str = "__str_get_len__";
pub const STR_STORE_BYTE_NAME: &str = "__str_store_byte__";
pub const STR_CONCAT_NAME: &str = "__str_concat__";
pub const STR_INDEX_NAME: &str = "__str_index__";
pub const STR_CHAR_CODE_AT_NAME: &str = "__str_char_code_at__";

// =====        nat/arrays.ej          =====
pub const ARR_ALLOCATE_NAME : &str = "__arr_alloc__";
pub const ARR_STORE_LENGTH_NAME: &str = "__arr_store_len__";
pub const ARR_STORE_CAPACITY_NAME: &str = "__arr_store_cap__";
pub const ARR_GET_LEN_NAME: &str = "__arr_get_len__";
pub const ARR_GET_CAP_NAME: &str = "__arr_get_cap__";
pub const ARR_REALLOCATE_NAME: &str = "__arr_reallocate__";
pub const ARR_PUSH_INT_NAME: &str = "__arr_push_int__";
pub const ARR_PUSH_FLOAT_NAME: &str = "__arr_push_float__";
pub const ARR_PUSH_STRING_NAME: &str = "__arr_push_string__";
pub const ARR_PUSH_ARRAY_NAME: &str = "__arr_push_array__";
pub const ARR_GET_ITEM_NAME: &str = "__arr_get_item__";