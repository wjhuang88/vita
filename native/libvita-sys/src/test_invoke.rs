use crate::common::{JString, JTestStruct};

#[no_mangle]
pub extern "system" fn hello_world() {
    println!("Hello World from Rust!");
}

#[no_mangle]
pub extern "system" fn test_add_int(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "system" fn test_struct(a: i32, b: i64, c: f32, d: f64) -> JTestStruct {
    JTestStruct { a, b, c, d }
}

#[no_mangle]
pub extern "system" fn test_struct_pointer(a: i32, b: i64, c: f32, d: f64) -> *const JTestStruct {
    let struct_box = Box::new(JTestStruct { a, b, c, d });
    Box::into_raw(struct_box)
}

#[no_mangle]
pub unsafe extern "system" fn test_struct_pointer_free(ptr: *mut JTestStruct) {
    println!("Freeing JTestStruct from Rust!");
    _ = Box::from_raw(ptr);
}

#[no_mangle]
pub extern "system" fn test_create_bytes() -> *const u8 {
    let items: Vec<u8> = b"Hello from Rust!".to_vec();
    items.leak().as_ptr()
}

#[no_mangle]
pub extern "system" fn test_create_string() -> *const JString {
    let items: Vec<u8> = b"Hello from Rust!".to_vec();
    println!("r: {}", items.as_ptr().addr());
    Box::into_raw(Box::new(JString {
        size: 16,
        data: items.leak().as_ptr()
    }))
}

#[no_mangle]
pub unsafe extern "system" fn test_create_string_free(ptr: *mut u8, size: usize) {
    println!("Freeing string from Rust!");
    _ = Vec::from_raw_parts(ptr, size, size);
}