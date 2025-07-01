mod comms;
use comms::{configure_tcp, control_sherhi};

use actix_files as fs;
use actix_web::{middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct Command {
    action: String,
}

async fn control_request(stream: Arc<Mutex<TcpStream>>, request: char) {
    let mut stream = stream.lock().unwrap();
    if let Err(e) = control_sherhi(&mut stream, request) {
        eprintln!("Error sending request to Arduino: {:?}", e);
    }
}

#[post("/control")]
async fn control(
    command: web::Json<Command>,
    stream: web::Data<Arc<Mutex<TcpStream>>>,
) -> impl Responder {
    let action = &command.action;
    match action.as_str() {
        "forward" => {
            println!("Forwarding SheRhi...");
            control_request(stream.get_ref().clone(), '+').await;
        }
        "reverse" => {
            println!("Reversing SheRhi...");
            control_request(stream.get_ref().clone(), '-').await;
        }
        "left" => {
            println!("SheRhi moving left...");
            control_request(stream.get_ref().clone(), 'l').await;
        }
        "right" => {
            println!("SheRhi moving right...");
            control_request(stream.get_ref().clone(), 'r').await;
        }
        "rotate_left" => {
            println!("SheRhi rotating left...");
            control_request(stream.get_ref().clone(), 'a').await;
        }
        "rotate_right" => {
            println!("SheRhi rotating right...");
            control_request(stream.get_ref().clone(), 'c').await;
        }
        "faster" => {
            println!("SheRhi speeding up...");
            control_request(stream.get_ref().clone(), 'f').await;
        }
        "slower" => {
            println!("SheRhi slowing down...");
            control_request(stream.get_ref().clone(), 's').await;
        }
        "stop" => {
            println!("Stopping SheRhi...");
            control_request(stream.get_ref().clone(), 'x').await;
        }
        _ => {
            eprintln!("Invalid action: {}", action);
        }
    }
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let stream = configure_tcp().expect("Failed to configure TCP connection");
    let shared_stream = Arc::new(Mutex::new(stream));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(shared_stream.clone()))
            .service(control)
            .service(fs::Files::new("/", "./static/html").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
