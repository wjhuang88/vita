use crate::common::{JBuffer, JTestStruct};

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
pub unsafe extern "system" fn test_create_bytes_free(ptr: *mut u8) {
    println!("Freeing bytes from Rust!");
    _ = Vec::from_raw_parts(ptr, 16, 16);
}

#[no_mangle]
pub extern "system" fn test_create_string(name_ptr: *mut u8, size: i32) -> *const JBuffer {
    let name = unsafe {
        Vec::from_raw_parts(name_ptr, size as usize, size as usize)
    };
    let items: Vec<u8> = [b"Hello ".to_vec(), name, " from Rust! 我们都是好朋友😄".as_bytes().to_vec()].concat();
    println!("r: {}", items.as_ptr().addr());
    Box::into_raw(Box::new(JBuffer {
        size: items.len() as i32,
        data: items.leak().as_ptr()
    }))
}

#[no_mangle]
pub unsafe extern "system" fn test_create_string_free(ptr: *mut u8, size: usize) {
    println!("Freeing string content from Rust!");
    _ = Vec::from_raw_parts(ptr, size, size);
}

#[no_mangle]
pub unsafe extern "system" fn test_create_string_wrapper_free(ptr: *mut JBuffer) {
    println!("Freeing JString from Rust!");
    _ = Box::from_raw(ptr);
}

type Callback = extern "system" fn(JBuffer) -> bool;

#[no_mangle]
pub extern "system" fn test_invoke_callback(callback: Callback) -> bool {
    let items: Vec<u8> = b"Hello from Rust Callback!".to_vec();
    let buffer = JBuffer {
        size: items.len() as i32,
        data: items.leak().as_ptr()
    };
    callback(buffer)
}