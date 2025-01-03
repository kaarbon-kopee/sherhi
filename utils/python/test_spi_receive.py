import spidev
import time

# Initialize SPI
spi = spidev.SpiDev()
spi.open(0, 0)               # Open SPI0 (CE0)
spi.max_speed_hz = 125000     # Set a low speed for stability
spi.mode = 0                  # Set SPI mode to 0

try:
    while True:
        data_to_send = 0x42   # Example byte to send
        received_data = spi.xfer([data_to_send])[0]  # Send data to Arduino and read response
        print("Sent:", hex(data_to_send), "| Received from Arduino:", hex(received_data))
        time.sleep(1)
except KeyboardInterrupt:
    spi.close()

