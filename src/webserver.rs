use tiny_http::{Server, Response};
use std::sync::{Arc, Mutex};
use crate::app::App;
use std::thread;

pub fn start_http_server(app: Arc<Mutex<App>>) {
    thread::spawn(move || {
        let server = Server::http("0.0.0.0:1337").unwrap();

        for request in server.incoming_requests() {
            if request.url() == "/processes" {
                let app_guard = app.lock().unwrap();
                let json = serde_json::to_string(&app_guard.processes).unwrap();
                let response = Response::from_string(json)
                .with_header(tiny_http::Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"application/json"[..]
                ).unwrap());
                request.respond(response).unwrap();
            } else {
                let response = Response::from_string("Hello from Rust HTTP server!");
                request.respond(response).unwrap();
            }
        }
    });
}