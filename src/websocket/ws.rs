use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use chrono::prelude::Local;
use log::{debug, info};
use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
///
const HEARTBEAT_TIMES: i32 = 36;
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  debug!("{:?}", r);
  let res = ws::start(WebSocket::new(), &r, stream);
  debug!("{:?}", res);
  res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct WebSocket {
  /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
  /// otherwise we drop connection.
  hb: Instant,
  try_times: i32,
}

impl Actor for WebSocket {
  type Context = ws::WebsocketContext<Self>;

  /// Method is called on actor start. We start the heartbeat process here.
  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);
  }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    // process websocket messages
    debug!("WS: {:?}", msg);
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        self.hb = Instant::now();
        ctx.pong(&msg);
      }
      Ok(ws::Message::Pong(_)) => {
        self.hb = Instant::now();
      }
      Ok(ws::Message::Text(text)) => ctx.text(format!("[{}]: {}", Local::now(), text)),
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      Ok(ws::Message::Close(_)) => {
        ctx.stop();
      }
      _ => ctx.stop(),
    }
  }
}

impl WebSocket {
  fn new() -> Self {
    Self {
      hb: Instant::now(),
      try_times: 0,
    }
  }

  /// helper method that sends ping to client every second.
  ///
  /// also this method checks heartbeats from client
  fn hb(&self, ctx: &mut <Self as Actor>::Context) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
      // check client heartbeats
      if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
        // heartbeat timed out
        info!("Websocket Client heartbeat failed, disconnecting!");

        // stop actor
        ctx.stop();

        // don't try to send a ping
        return;
      }
      if act.try_times > HEARTBEAT_TIMES {
        info!(
          "heartbeat {} times, no write and read action occurs, close the connect!",
          act.try_times
        );
        // stop actor
        ctx.stop();
        return;
      }
      debug!("send heartbeat to client {} time", act.try_times);
      ctx.ping(b"");
      act.try_times += 1;
    });
  }
}
