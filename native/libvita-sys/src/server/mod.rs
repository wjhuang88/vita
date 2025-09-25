mod callback;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::StreamExt;
use uuid::Uuid;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender as Sender};

use crate::common::JBuffer;

pub use callback::*;

#[no_mangle]
pub extern "system" fn start_vita_server() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // 启动 HttpServer
        let server = HttpServer::new(move || {

            App::new()
                .default_service(web::to(main_handle))
        })
        .bind(("127.0.0.1", 3000))
        .unwrap()
        .run();

        // 启动服务
        server.await.unwrap()
    })
}

async fn main_handle(req: HttpRequest, mut body: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let (send, mut recv) = unbounded_channel::<JBuffer>();
    if let Some(handle) = start_request(req.path().to_string(), Box::into_raw(Box::new(send))) {
        while let Some(chunk) = body.next().await {
            let chunk = chunk?;
            let chunk_ptr = chunk.as_ptr();
            let len = chunk.len() as i32;
            let input = JBuffer::external_data(Uuid::new_v4(), len, chunk_ptr);
            let ptr = Box::into_raw(Box::new(input));
            (handle.push_handle)(ptr);
        }
        extern "system" fn resp(resp_ptr: *const JBuffer, send: *const Sender<JBuffer>) {
            let resp = unsafe { Box::from_raw(resp_ptr as *mut JBuffer) };
            let sender = unsafe { Box::from_raw(send as *mut Sender<JBuffer>) };
            sender.send(*resp).unwrap();
        }
        (handle.end_handle)(resp);
        recv.recv().await.map(|buf| {
            let body = buf.to_vec();
            HttpResponse::Ok().body(body)
        }).ok_or_else(|| {
            actix_web::error::ErrorInternalServerError("No response received")
        })
    } else {
        Err(actix_web::error::ErrorNotFound(format!("No request handle registered for path: {}", req.path())))
    }
}

