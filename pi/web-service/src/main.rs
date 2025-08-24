// Module for TCP communication with Arduino
mod comms;
use comms::{configure_tcp, control_sherhi};

use actix_files as fs;
use actix_web::{middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::process::Command;

/// Represents a user command received via HTTP POST (e.g., "forward", "stop").
#[derive(Deserialize)]
struct UserCommand {
    action: String,
}

/// Sends a single-character command to the Arduino over a shared TCP stream.
///
/// # Arguments
/// * `stream` - Shared and mutex-guarded TCP stream.
/// * `request` - Character command to send to the Arduino.
async fn control_request(stream: Arc<Mutex<TcpStream>>, request: char) {
    let mut stream = stream.lock().unwrap();
    if let Err(e) = control_sherhi(&mut stream, request) {
        eprintln!("Error sending request to Arduino: {:?}", e);
    }
}

/// Handles incoming control requests from the UI and maps them to TCP commands.
///
/// Endpoint: POST /control
#[post("/control")]
async fn control(
    command: web::Json<UserCommand>,
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


/// Synthesizes and plays a spoken message using `pico2wave` and `paplay`.
///
/// Endpoint: POST /say
#[post("/say")]
async fn say(message: web::Json<UserCommand>) -> impl Responder {
    let text = &message.action;

    // Construct shell command to generate and play audio
    let cmd = format!(
        "pico2wave -w /tmp/hello.wav '{}' && paplay /tmp/hello.wav",
        text
    );

    // Execute the shell command
    let result = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output();

    // Handle success or error output
    match result {
        Ok(result) if result.status.success() => {
            println!("Spoken: {}", text);
        }
        Ok(output) => {
            eprintln!(
                "Error speaking text:\nstderr: {}\nstdout: {}",
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
    "OK"
}

/// Main entry point to the Actix-Web service.
/// - Initializes logging
/// - Sets audio volume
/// - Connects to Arduino
/// - Starts HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger from environment variable (e.g., RUST_LOG)
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Set speaker volume at startup (sink 80 to 100%)
    let _ = std::process::Command::new("pactl")
        .args(["set-sink-volume", "80", "100%"])
        .output();

    // Establish TCP connection with the Arduino
    let stream = configure_tcp().expect("Failed to configure TCP connection");
    let shared_stream = Arc::new(Mutex::new(stream));

    // Start HTTP server with shared state and registered routes
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(shared_stream.clone()))
            .service(control)
            .service(say)
            .service(fs::Files::new("/", "./static/html").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
