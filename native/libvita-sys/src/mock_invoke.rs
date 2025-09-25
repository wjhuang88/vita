use std::mem::ManuallyDrop;

use uuid::Uuid;

use crate::common::JBuffer;

#[repr(C)]
pub struct JTestStruct {
    pub a: i32,
    pub b: i64,
    pub c: f32,
    pub d: f64,
}

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
    let name = ManuallyDrop::new(name);
    let items: Vec<u8> = [b"Hello ".to_vec(), name.to_vec(), " from Rust! 我们都是好朋友😄".as_bytes().to_vec()].concat();
    let b = Box::new(JBuffer::new(Uuid::new_v4(), items.len() as i32, items.leak().as_ptr()));
    let r = Box::into_raw(b);
    println!("Created JBuffer from Rust: {:?}, managed: {}, id addr: {}", r.addr(), unsafe { &*r }.managed, unsafe { &*r }.request_id.addr());
    r
}

type Callback = extern "system" fn(*mut JBuffer) -> bool;

#[no_mangle]
pub extern "system" fn test_invoke_callback(callback: Callback) -> bool {
    let items: Vec<u8> = b"Hello from Rust Callback!".to_vec();
    let buffer = JBuffer::new(Uuid::new_v4(), items.len() as i32, items.leak().as_ptr());
    let ptr: *mut JBuffer = Box::into_raw(Box::new(buffer));
    callback(ptr)
}