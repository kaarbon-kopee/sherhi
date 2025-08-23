//! # SheRhi Control Program
//!
//! This program connects to the Arduino controller over TCP and
//! allows the user to send movement commands interactively.
//!
//! Supported commands include forward, reverse, left, right,
//! rotation, speed control, and stop.
use web_service::comms::{configure_tcp, control_sherhi};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    /// Entry point for the SheRhi control program.
    //
    /// This function:
    /// - Connects to the Arduino via TCP
    /// - Reads user input commands interactively
    /// - Sends validated commands to the Arduino
    ///
    /// Returns an [`io::Result`] which is `Ok(())` on success or an error
    /// if the TCP connection or IO fails.
    let mut stream = configure_tcp()?;

    loop {
        // Request to start or stop SheRhi
        print!("Enter request to start or stop SheRhi: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // single character input
        let request: char = match input.chars().next() {
            Some('q') | Some('Q') => {
                println!("Exiting SheRhi control...");
                break;
            }
            Some(c)
                if c == '+'
                    || c == '-'
                    || c == 'l'
                    || c == 'r'
                    || c == 'a'
                    || c == 'c'
                    || c == 'f'
                    || c == 'F'
                    || c == 's'
                    || c == 'S'
                    || c == 'x'
                    || c == 'X' =>
            {
                c
            }
            _ => {
                eprintln!(
                    "Invalid input. Please enter:\n\
                    '+' for forward,\n\
                    '-' for reverse,\n\
                    'l' for left,\n\
                    'r' for right,\n\
                    'a' for rotate left,\n\
                    'c' for rotate right,\n\
                    'f' or 'F' to speed up,\n\
                    's' or 'S' to slow down,\n\
                    'x' or 'X' to stop SheRhi,\n\
                    'q' or 'Q' to quit."
                );
                continue;
            }
        };

        // Send request to Arduino and validate response
        match control_sherhi(&mut stream, request) {
            Ok(_) => println!("Request sent to Arduino."),
            Err(e) => eprintln!("Error sending request to Arduino: {:?}", e),
        }
    }

    Ok(())
}
