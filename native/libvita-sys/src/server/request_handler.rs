use crate::common::JBuffer;

use tokio_stream::Stream;
use futures::StreamExt;
use std::sync::LazyLock;
use std::mem::forget;
use tokio::sync::mpsc::UnboundedSender as Sender;


pub type StartHandle = extern "system" fn(*mut Sender<*mut JBuffer>) -> *const RequestHandleEntry;

static HANDLE_MAP: LazyLock<dashmap::DashMap<String, StartHandle>> = LazyLock::new(|| dashmap::DashMap::new());

pub(crate) struct ResponseStream<S> {
    pub(crate) inner: S,
    pub(crate) close_handle: extern "system" fn()
}

impl<S: Stream + std::marker::Unpin> Stream for ResponseStream<S> {
    type Item = S::Item;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.inner.poll_next_unpin(cx)
    }
}

impl<S> Drop for ResponseStream<S> {
    fn drop(&mut self) {
        (self.close_handle)();
    }
}

#[repr(C)]
pub struct RequestHandleEntry {
    pub push_handle: extern "system" fn(*mut JBuffer),
    pub end_request_handle: extern "system" fn(extern "system" fn(*mut JBuffer, *mut Sender<*mut JBuffer>), extern "system" fn(*mut Sender<*mut JBuffer>)),
    pub close_handle: extern "system" fn()
}

#[no_mangle]
pub extern "system" fn register_request_handle(path_buf: *const JBuffer, start_handle: StartHandle) {
    let path = unsafe { (&*path_buf).to_string() };
    HANDLE_MAP.insert(path.clone(), start_handle);
    forget(path);
}

pub(crate) fn start_request(path: String, send: *mut Sender<*mut JBuffer>) -> Option<Box<RequestHandleEntry>> {
    let wrap = HANDLE_MAP.get(&path);
    wrap.map(|start_handle| start_handle(send))
        .map(|ptr| unsafe { Box::from_raw(ptr as *mut RequestHandleEntry) })
}