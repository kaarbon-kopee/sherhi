import spidev
import time

spi = spidev.SpiDev()  # Create SPI object
spi.open(0, 0)         # Open SPI bus 0, device 0
spi.max_speed_hz = 2000000  # Set SPI clock speed to 100 kHz

print("Generating SCK clock signal...")
try:
    while True:
        spi.xfer2([0x23])  # Send dummy data to generate SCK activity
        time.sleep(0.1)    # Small delay between transfers
except KeyboardInterrupt:
    print("Stopped")
    spi.close()

