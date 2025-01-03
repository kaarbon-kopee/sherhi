import spidev
import RPi.GPIO as GPIO
import time

# SPI setup
spi = spidev.SpiDev()
spi.open(0, 0)  # SPI bus 0, device 0 (CE0)
spi.max_speed_hz = 2000000
spi.mode = 0  # SPI mode 0 (CPOL=0, CPHA=0)

# GPIO setup for SS pin
GPIO.setmode(GPIO.BCM)
GPIO.setup(8, GPIO.OUT)  # GPIO 8 = CE0 (SS)

try:
    while True:
        print("Starting SPI transaction...")  # Debug message
        GPIO.output(8, GPIO.LOW)  # SS LOW (Activate slave)
        time.sleep(0.001)  # Small delay for synchronization
        response = spi.xfer2([0xA])  # Send 3 bytes
        GPIO.output(8, GPIO.HIGH)  # SS HIGH (Deactivate slave)
        print(f"Received: {response}")  # Print response from Arduino
        time.sleep(1)  # Wait 1 second before next transaction
finally:
    GPIO.cleanup()
    spi.close()

