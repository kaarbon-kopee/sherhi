mod comms;
use comms::{configure_tcp, control_sherhi};

use actix_files as fs;
use actix_web::{middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;
use std::process::Command;

/// Incoming HTTP request payload
#[derive(Deserialize)]
struct UserCommand {
    action: String,
}

/// Open a TCP connection and send a command to the Arduino.
async fn control_request(request: char) {
    match configure_tcp() {
        Ok(mut stream) => {
            if let Err(e) = control_sherhi(&mut stream, request) {
                eprintln!("Error sending request to Arduino: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to Arduino: {:?}", e);
        }
    }
}

/// HTTP POST /control
#[post("/control")]
async fn control(command: web::Json<UserCommand>) -> impl Responder {
    let action = command.action.as_str();

    match action {
        "forward" => control_request('+').await,
        "reverse" => control_request('-').await,
        "left" => control_request('l').await,
        "right" => control_request('r').await,
        "rotate_left" => control_request('a').await,
        "rotate_right" => control_request('c').await,
        "faster" => control_request('f').await,
        "slower" => control_request('s').await,
        "stop" => control_request('x').await,
        _ => eprintln!("Invalid action: {}", action),
    }

    "OK"
}

#[post("/say")]
async fn say(message: web::Json<UserCommand>) -> impl Responder {
    let text = &message.action;

    // Volume: 0–200 (100 = default, 150 = louder)
    let result = Command::new("espeak")
        .args(["-ven+f3", "-a200", text])
        .output();

    match result {
        Ok(out) if out.status.success() => println!("Spoken: {}", text),
        Ok(out) => {
            eprintln!(
                "Speech error:\nstderr: {}\nstdout: {}",
                String::from_utf8_lossy(&out.stderr),
                String::from_utf8_lossy(&out.stdout)
            );
        }
        Err(e) => eprintln!("Failed to execute TTS: {}", e),
    }

    "OK"
}

/// Entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Set audio volume (safe if it fails)
    let _ = Command::new("pactl")
        .args(["set-sink-volume", "80", "100%"])
        .output();

    println!("Starting SheRhi web server on port 8080...");

    // Web server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(control)
            .service(say)
            .service(fs::Files::new("/", "/opt/sherhi/html").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}