use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::io;

pub fn configure_spi() -> io::Result<Spidev> {
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

pub fn control_sherhi(spi: &mut Spidev, request: char) -> io::Result<()> {
    let tx_buf: [u8; 1] = [request as u8]; // Command to send to Arduino

    // Set up the transfer
    let mut transfer = SpidevTransfer::write(&tx_buf);
    spi.transfer(&mut transfer)?;

    Ok(())
}
