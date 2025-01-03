use std::io::{self, Write};
use std::net::TcpStream;

pub fn control_sherhi(stream: &mut TcpStream, request: char) -> io::Result<()> {
    let request_str = format!("{}\n", request);
    println!("Sending request to Arduino: {}", request_str.trim());
    stream.write_all(request_str.as_bytes())?;
    Ok(())
}

pub fn configure_tcp() -> io::Result<TcpStream> {
    let arudino_ip = "192.168.2.2";
    let arduino_port = 5000;

    let connection_address = format!("{}:{}", arudino_ip, arduino_port);
    println!("Connecting to Arduino at: {}", connection_address);

    // Attempt to connect to the Arduino
    TcpStream::connect(&connection_address)
}
