use comms::{configure_spi, control_sherhi};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Configure the SPI interface
    let mut spi = configure_spi()?;

    loop {
        // Request to start or stop SheRhi
        print!("Enter request to start or stop SheRhi:");
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
            Some(c) if c == '+' || c == '-' || c == 's' || c == 'S' => c,
            _ => {
                eprintln!(
                    "Invalid input. Please enter:\n\
                    '+' for forward,\n\
                    '-' for reverse,\n\
                    's' or 'S' to stop SheRhi,\n\
                    'q' or 'Q' to quit."
                );
                continue;
            }
        };

        // Send request to Arduino and validate response
        let sensor_value = control_sherhi(&mut spi, request)?;
        println!("Echo value from Arduino: {:?}", sensor_value);
    }

    Ok(())
}
