import spidev
import time

spi = spidev.SpiDev()  # Create an SPI object
spi.open(0, 0)         # Open SPI bus 0, device 0
spi.max_speed_hz = 50000  # Set SPI clock speed
spi.mode = 0           # SPI mode 0

print("Starting SPI loopback test...")
try:
    while True:
        send_data = [0x42, 0x37, 0xAA]  # Data to send
        received_data = spi.xfer2(send_data)  # Send and receive data
        print(f"Sent: {send_data} | Received: {received_data}")
        time.sleep(1)  # Wait 1 second
except KeyboardInterrupt:
    print("Test stopped.")
    spi.close()

