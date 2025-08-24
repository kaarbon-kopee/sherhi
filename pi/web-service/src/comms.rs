use std::io::{self, Write};
use std::net::TcpStream;

/// Sends a single-character command to the Arduino over TCP.
///
/// # Arguments
/// * `stream` - A mutable reference to an established `TcpStream`.
/// * `request` - A character representing the command (e.g., 'f' for forward).
///
/// # Returns
/// * `Ok(())` if the command was successfully written to the stream.
/// * `Err(io::Error)` if writing to the stream fails.
pub fn control_sherhi(stream: &mut TcpStream, request: char) -> io::Result<()> {
    // Format the character as a string with a newline (Arduino expects newline-terminated commands)
    let request_str = format!("{}\n", request);
    
    println!("Sending request to Arduino: {}", request_str.trim());

    // Write the command to the stream
    stream.write_all(request_str.as_bytes())?;
    Ok(())
}

/// Establishes a TCP connection to the Arduino using a fixed IP and port.
///
/// # Returns
/// * `Ok(TcpStream)` if the connection is successful.
/// * `Err(io::Error)` if the connection attempt fails.
pub fn configure_tcp() -> io::Result<TcpStream> {
    let arudino_ip = "192.168.2.2";
    let arduino_port = 5000;

    let connection_address = format!("{}:{}", arudino_ip, arduino_port);
    println!("Connecting to Arduino at: {}", connection_address);

    // Attempt to connect to the Arduino
    TcpStream::connect(&connection_address)
}
