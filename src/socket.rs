use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use rand::Rng;
use serde::{Deserialize, Serialize};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// custom and simple message format used in websocket communication
#[derive(Serialize, Deserialize)]
struct CalcTaskMessage {
    action: MessageAction,
    content: String,
}

#[derive(Serialize, Deserialize)]
enum MessageAction {
    Task,
    Result,
}

/// websocket connection is long running connection -> handle with actor
pub struct CalcTaskWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    task_result: i32,
    rng: rand::rngs::ThreadRng
}

impl CalcTaskWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now(), task_result: 0, rng: rand::thread_rng() }
    }

    /// handle incoming messages and create appropriate responses
    fn handle_message(&mut self, msg: &CalcTaskMessage, ctx: &mut <Self as actix::Actor>::Context) {
    
        match msg.action {
            MessageAction::Task => {

                let left = self.rng.gen_range(1..20);
                let right = self.rng.gen_range(1..20);
                let op = if self.rng.gen_bool(0.5) {
                    self.task_result = left + right;
                    '+'
                } else { 
                    self.task_result = left - right;
                    '-'
                };
    
                let calc_task_msg = CalcTaskMessage {
                    action: MessageAction::Task,
                    content: format!("{left} {op} {right}"),
                };

                ctx.text(serde_json::to_string(&calc_task_msg).unwrap());
            },
            MessageAction::Result => {

                let calc_task_msg = CalcTaskMessage {
                    action: MessageAction::Result,
                    content: if msg.content == self.task_result.to_string() { String::from("correct") } else { String::from("false") },
                };
    
                ctx.text(serde_json::to_string(&calc_task_msg).unwrap());
            }
        }
    }
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {

            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                // stop actor
                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for CalcTaskWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for CalcTaskWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let parsed_msg_result: serde_json::Result<CalcTaskMessage> = serde_json::from_str(text.to_string().as_str());
                
                match parsed_msg_result {
                    Ok(msg) => self.handle_message(&msg, ctx),
                    Err(e) => log::info!("Error while parsing JSON in web socket: {}", e)
                }
                
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }

    
}