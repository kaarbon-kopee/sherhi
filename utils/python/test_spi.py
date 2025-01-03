import spidev
import time

spi = spidev.SpiDev()
spi.open(0, 0)              # Use SPI0 (CE0)
spi.max_speed_hz = 100000    # Start with 500 kHz
spi.mode = 0                 # Set SPI mode to Mode 0

try:
    while True:
        data_to_send = 0x42  # Example byte to send
        spi.xfer([data_to_send])  # Send data to Arduino
        print("Sent data:", data_to_send)
        time.sleep(1)
except KeyboardInterrupt:
    spi.close()

