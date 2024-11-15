use actix_files as fs;
use actix_web::{middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;

use comms;

#[derive(Deserialize)]
struct Command {
    action: String,
}

async fn control_request(request: char) {
    let mut spi = comms::configure_spi().unwrap();
    comms::control_sherhi(&mut spi, request).unwrap();
}

#[post("/control")]
async fn control(command: web::Json<Command>) -> impl Responder {
    let action = &command.action;
    match action.as_str() {
        "forward" => {
            println!("Forwarding SheRhi...");
            control_request('+').await;
        }
        "reverse" => {
            println!("Reversing SheRhi...");
            control_request('-').await;
        }
        "left" => {
            println!("SheRhi moving left...");
            control_request('l').await;
        }
        "right" => {
            println!("SheRhi moving right...");
            control_request('r').await;
        }
        "stop" => {
            println!("Stopping SheRhi...");
            control_request('s').await;
        }
        _ => {
            println!("Invalid action: {}", action);
        }
    }
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(control)
            .service(fs::Files::new("/", "./static/html").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
