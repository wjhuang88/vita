use std::{mem::forget, ops::Deref};

use uuid::Uuid;

pub static ID_LEN : usize = 16;

#[repr(C)]
pub struct JBuffer {
    pub size: i32,
    pub managed: bool,
    pub data_managed: bool,
    pub request_id: *const u8,
    pub data: *const u8,
}

impl JBuffer {

    pub fn new(request_id: Uuid, size: i32, data: *const u8) -> Self {
        let id_ptr = request_id.as_bytes().to_vec().leak().as_ptr();
        JBuffer {
            size,
            managed: true,
            data_managed: true,
            request_id: id_ptr,
            data,
        }
    }

    pub fn external_data(request_id: Uuid, size: i32, data: *const u8) -> Self {
        let id_ptr = request_id.as_bytes().to_vec().leak().as_ptr();
        JBuffer {
            size,
            managed: true,
            data_managed: false,
            request_id: id_ptr,
            data,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.size as usize) }
    }
}

impl ToString for JBuffer {
    fn to_string(&self) -> String {
        unsafe {
            String::from_raw_parts(self.data.cast_mut(), self.size as usize, self.size as usize)
        }
    }
}

impl Deref for JBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl AsRef<[u8]> for JBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Drop for JBuffer {
    fn drop(&mut self) {
        unsafe {
            if self.managed {
                let _ = Vec::from_raw_parts(self.request_id.cast_mut(), ID_LEN, ID_LEN);
            }
            if self.data_managed {
                let _ = Vec::from_raw_parts(self.data.cast_mut(), self.size as usize, self.size as usize);
            }
        }
    }
    
}

#[no_mangle]
pub unsafe extern "system" fn free_jbuffer(ptr: *mut JBuffer) {
    let buf = Box::from_raw(ptr);
    if !buf.managed {
        forget(buf);
    }
}