use std::sync::Mutex;

use actix::{Actor, StreamHandler};
use actix::prelude::*;
use actix_web::{Error, get, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use once_cell::sync::Lazy;
use tracing::info;

use crate::infra::Infra;

static WS_ADDR: Lazy<Mutex<Option<Addr<MyWebSocket>>>> = Lazy::new(|| Mutex::new(None));

#[get("/active-actions")]
pub async fn active_actions() -> Result<HttpResponse, Error> {
    info!("Requesting current active actions");
    let tasks = Infra::tasks().get_all_active();
    let tasks_json = serde_json::to_string(&tasks).unwrap();
    Ok(HttpResponse::Ok().body(tasks_json))
}

#[get("/ws/")]
pub async fn active_actions_ws(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    info!("Websocket connection initiated");
    let resp = ws::start(MyWebSocket::new(), &req, stream);
    resp
}

#[derive(Clone)]
pub struct MyWebSocket {}

impl MyWebSocket {
    pub fn new() -> Self {
        MyWebSocket {}
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Register self in AppState.
        *WS_ADDR.lock().unwrap() = Some(ctx.address());
        self.send_new_tasks(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // Clear the address from AppState.
        info!("Websocket connection closed");
        *WS_ADDR.lock().unwrap() = None;
        Running::Stop
    }
}

pub struct TasksUpdate(pub String);

impl Message for TasksUpdate {
    type Result = ();
}

impl Handler<TasksUpdate> for MyWebSocket {
    type Result = ();
    fn handle(&mut self, msg: TasksUpdate, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(_)) => {
                // Handle incoming text messages if needed
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

// This is just an example. In a real-world scenario, you might want to use some event system or
// database triggers to notify of new tasks, and then push them to the client.
impl MyWebSocket {
    fn send_new_tasks(&self, ctx: &mut <Self as Actor>::Context) {
        let tasks = Infra::tasks().get_all_active();
        let tasks_json = serde_json::to_string(&tasks).unwrap();
        ctx.text(tasks_json);
    }
}

pub async fn send_new_tasks_to_ws() {
    let tasks = Infra::tasks().get_all_active();
    let tasks_json = serde_json::to_string(&tasks).unwrap();

    println!("addr chceck {:?}", WS_ADDR.lock().unwrap().as_ref());

    if let Some(addr) = WS_ADDR.lock().unwrap().as_ref() {
        addr.do_send(TasksUpdate(tasks_json));
    }
}
