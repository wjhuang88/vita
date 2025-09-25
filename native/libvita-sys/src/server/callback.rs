
use crate::common::JBuffer;

use std::mem::{forget};
use std::sync::LazyLock;
use tokio::sync::mpsc::UnboundedSender as Sender;

static HANDLE_MAP: LazyLock<dashmap::DashMap<String, StartHandle>> = LazyLock::new(|| dashmap::DashMap::new());

#[repr(C)]
pub struct RequestHandleEntry {
    pub push_handle: extern "system" fn(*const JBuffer),
    pub end_handle: extern "system" fn(extern "system" fn(*const JBuffer, *const Sender<JBuffer>)),
}

pub type StartHandle = extern "system" fn(*const Sender<JBuffer>) -> *const RequestHandleEntry;

#[no_mangle]
pub extern "system" fn register_request_handle(path_buf: *const JBuffer, start_handle: StartHandle) {
    let path = unsafe { (&*path_buf).to_string() };
    println!("Registering request handle for path buffer at: {:?}", path);
    HANDLE_MAP.insert(path.clone(), start_handle);
    forget(path);
}

pub(crate) fn start_request(path: String, send: *const Sender<JBuffer>) -> Option<Box<RequestHandleEntry>> {
    let wrap = HANDLE_MAP.get(&path);
    wrap.map(|start_handle| start_handle(send))
        .map(|ptr| unsafe { Box::from_raw(ptr as *mut RequestHandleEntry) })
}