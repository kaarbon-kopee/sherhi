use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::io::{self, Write};

fn configure_spi() -> io::Result<Spidev> {
    // Open the SPI device
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let mut options = SpidevOptions::new();

    // Set SPI options (e.g., mode, speed)
    options
        .bits_per_word(8)
        .max_speed_hz(1_000_000) // 1 MHz
        .mode(SpiModeFlags::SPI_MODE_0); // SPI Mode 0
                                         //.build();

    spi.configure(&options)?;
    Ok(spi)
}

fn start_stop_request_sherhi(spi: &mut Spidev, request: char) -> io::Result<()> {
    let tx_buf: [u8; 1] = [request as u8]; // Command to send to Arduino

    // Set up the transfer
    let mut transfer = SpidevTransfer::write(&tx_buf);
    spi.transfer(&mut transfer)?;

    Ok(())
}

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
        let sensor_value = start_stop_request_sherhi(&mut spi, request)?;
        println!("Echo value from Arduino: {:?}", sensor_value);
    }

    Ok(())
}
