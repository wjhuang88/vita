mod request_handler;

use std::mem::{forget, ManuallyDrop};

use actix_web::{http::header::ContentType, web, App, HttpRequest, HttpResponse, HttpServer};
use tokio_stream::wrappers::UnboundedReceiverStream;
use futures::StreamExt;
use uuid::Uuid;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender as Sender};

use crate::{common::JBuffer, server::request_handler::{StartHandle, HANDLE_MAP}};
use request_handler::{ResponseStream, start_request};

#[no_mangle]
pub extern "system" fn start_vita_server() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // 启动 HttpServer
        let server = HttpServer::new(move || {
            let mut app = App::new();
            for entry in HANDLE_MAP.iter() {
                let start_handler: StartHandle = *entry.value();
                app = app.route(&entry.key(), web::to(move |req: HttpRequest, body: web::Payload| async move {
                    main_handle(start_handler, req, body).await
                }));
            }
            app
        })
        .bind(("127.0.0.1", 3000))
        .unwrap()
        .run();

        // 启动服务
        server.await.unwrap()
    })
}

extern "system" fn send_response_ptr(resp_ptr: *mut JBuffer, send: *mut Sender<*mut JBuffer>) {
    let sender = unsafe { Box::from_raw(send) };
    sender.send(resp_ptr).unwrap();
    forget(sender);
}

extern "system" fn send_end_response_ptr(send: *mut Sender<*mut JBuffer>) {
    let _ = unsafe { Box::from_raw(send) };
}

async fn main_handle(handler: StartHandle, _req: HttpRequest, mut body: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let (send, recv) = unbounded_channel::<*mut JBuffer>();
    let handle_entry = start_request(handler, Box::into_raw(Box::new(send)));
    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        let chunk_ptr = chunk.as_ptr();
        let len = chunk.len() as i32;
        let input = JBuffer::external_data(Uuid::new_v4(), len, chunk_ptr);
        let ptr = Box::into_raw(Box::new(input));
        (handle_entry.push_handle)(ptr);
    }
    (handle_entry.end_request_handle)(send_response_ptr, send_end_response_ptr); // 由java端调用send_response_ptr并回传send实例
    let recv_stream = UnboundedReceiverStream::new(recv).map(|buf| {
            // 返回值会在ResponseStream的drop中发送释放信号到java端，然后释放
            let body = unsafe { ManuallyDrop::new(Box::from_raw(buf)) };
            let body = web::Bytes::from_static(body.as_statics());
            Ok::<_, actix_web::Error>(body)
        });
    let resp = HttpResponse::Ok().content_type(ContentType::plaintext()).streaming(ResponseStream { inner: recv_stream, close_handle: handle_entry.close_handle });
    // java端释放对象指针
    forget(handle_entry);
    Ok(resp)
}

