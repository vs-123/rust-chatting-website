extern crate ws;

use std::fs;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};

struct Server { // Server struct for implementing the websocket
    out: Sender,
}

impl Handler for Server { // Implement websocket handler
    // Handling requests and routes
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            // Implement the websocket route
            "/ws" => Response::from_request(req),

            // The main route where we will serve our html file
            "/" => Ok(
                Response::new(
                    200,
                    "OK",
                    fs::read_to_string("index.html")
                    .expect("Something went wrong reading the file")
                    .as_bytes()
                    .to_vec()
                )
            ),

            // Handle invalid routes
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    // Handle messages recieved on /ws
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast the received msg to all clients
        self.out.broadcast(msg)
    }
}

fn main() {
    // Listen on 127.0.0.1 (localhost) at port 8000 and make a Server struct for each client that gets connected
    listen("127.0.0.1:8000", |out| Server { out }).unwrap()
}
