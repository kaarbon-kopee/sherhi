import RPi.GPIO as GPIO
import spidev
import time

# Set up GPIO 8 (CE0)
GPIO.setmode(GPIO.BCM)
GPIO.setwarnings(False)
GPIO.setup(8, GPIO.OUT)

spi = spidev.SpiDev()
spi.open(0, 0)              # Use SPI0 (CE0)
spi.max_speed_hz = 2000000
spi.mode = 0                 # Set SPI mode to Mode 0

try:
    while True:
        GPIO.output(8, GPIO.LOW)  # Activate SS (Low)
        print("SS: LOW")
        response = spi.xfer2([0x01, 0x02, 0x03, 0x04])  # Send some data
        time.sleep(1)
        GPIO.output(8, GPIO.HIGH)  # Deactivate SS (High)
        print("SS: HIGH")
        print(f"Received: {response}")
        time.sleep(1)
finally:
    GPIO.cleanup()
