
use actix::{Actor, StreamHandler, Handler, Message as ActixMessage, Context, AsyncContext};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessageType {
    NewMessage,
    NewNotification,
    ProjectUpdate,
    MilestoneUpdate,
    PaymentUpdate,
    ProposalUpdate,
    JobUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub message_type: WsMessageType,
    pub data: serde_json::Value,
    pub user_id: i32,
    pub timestamp: i64,
}

pub struct WebSocketSession {
    pub user_id: i32,
    pub connections: Arc<Mutex<HashMap<i32, Vec<actix::Addr<WebSocketSession>>>>>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut conns = self.connections.lock().unwrap();
        conns.entry(self.user_id)
            .or_insert_with(Vec::new)
            .push(ctx.address());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let mut conns = self.connections.lock().unwrap();
        if let Some(user_conns) = conns.get_mut(&self.user_id) {
            user_conns.retain(|addr| addr != &ctx.address());
            if user_conns.is_empty() {
                conns.remove(&self.user_id);
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // Handle incoming messages from client
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    self.broadcast_to_user(ws_msg);
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl WebSocketSession {
    pub fn new(user_id: i32, connections: Arc<Mutex<HashMap<i32, Vec<actix::Addr<WebSocketSession>>>>>) -> Self {
        Self { user_id, connections }
    }

    pub fn broadcast_to_user(&self, message: WsMessage) {
        let conns = self.connections.lock().unwrap();
        if let Some(user_conns) = conns.get(&message.user_id) {
            let msg_json = serde_json::to_string(&message).unwrap();
            for conn in user_conns {
                conn.do_send(BroadcastMessage(msg_json.clone()));
            }
        }
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

impl Handler<BroadcastMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub struct WebSocketManager {
    connections: Arc<Mutex<HashMap<i32, Vec<actix::Addr<WebSocketSession>>>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_connections(&self) -> Arc<Mutex<HashMap<i32, Vec<actix::Addr<WebSocketSession>>>>> {
        Arc::clone(&self.connections)
    }

    pub fn send_to_user(&self, user_id: i32, message: WsMessage) {
        let conns = self.connections.lock().unwrap();
        if let Some(user_conns) = conns.get(&user_id) {
            let msg_json = serde_json::to_string(&message).unwrap();
            for conn in user_conns {
                conn.do_send(BroadcastMessage(msg_json.clone()));
            }
        }
    }

    pub fn broadcast(&self, message: WsMessage) {
        let conns = self.connections.lock().unwrap();
        let msg_json = serde_json::to_string(&message).unwrap();
        for user_conns in conns.values() {
            for conn in user_conns {
                conn.do_send(BroadcastMessage(msg_json.clone()));
            }
        }
    }
}
