#[repr(C)]
pub struct JTestStruct {
    pub a: i32,
    pub b: i64,
    pub c: f32,
    pub d: f64,
}

#[repr(C)]
pub struct JBuffer {
    pub size: i32,
    pub data: *const u8,
}