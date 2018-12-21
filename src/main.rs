#[macro_use]
extern crate serde_derive;
use actix::*;
use actix_web::{server, ws, App};
use lz4::Decoder;
use std::io;

/// Define http actor
struct Ws;

impl Default for Ws {
    fn default() -> Self {
        Self {}
    }
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(_text) => (),
            ws::Message::Binary(mut bin) => {
                let bytes = bin.take().to_vec();
                let mut decoder = Decoder::new(&bytes[..]).expect("Unable to create decoder");
                let mut output: Vec<u8> = Vec::new();
                io::copy(&mut decoder, &mut output)
                    .expect("Unable to copy data from decoder to output buffer");
                print!("{}", String::from_utf8(output).unwrap());
            }
            _ => {
                println!("Unknown message");
                ()
            }
        }
    }
}

use docopt::Docopt;

const USAGE: &'static str = r#"
Compressed log sink.

Usage:
  compressed_log_sink --bind=<address>
  compressed_log_sink (-h | --help)
  compressed_log_sink --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --bind=<address>  Bind to address [default: 0.0.0.0:9999].
"#;

#[derive(Debug, Deserialize)]
struct Args {
    flag_bind: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    server::new(|| {
        App::new()
            .resource("/sink/", |r| {
                r.f(|req| {
                    println!("Something happened!");
                    ws::start(req, Ws::default())
                })
            })
            .finish()
    })
    .bind(args.flag_bind.clone())
    .unwrap_or_else(|_| panic!("Unable to bind to {}", args.flag_bind))
    .run();
}
