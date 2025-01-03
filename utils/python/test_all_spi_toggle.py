import RPi.GPIO as GPIO
import time

# Define SPI pins
MOSI = 10
MISO = 9  # Not tested as output, only input
SCLK = 11
SS = 8

# Set up GPIO
GPIO.setmode(GPIO.BCM)
GPIO.setwarnings(False)

# Set SPI output pins
GPIO.setup(MOSI, GPIO.OUT)
GPIO.setup(SCLK, GPIO.OUT)
GPIO.setup(SS, GPIO.OUT)

try:
    while True:
        # Test MOSI
        GPIO.output(MOSI, GPIO.HIGH)
        print("MOSI: HIGH")
        time.sleep(0.5)
        GPIO.output(MOSI, GPIO.LOW)
        print("MOSI: LOW")
        time.sleep(0.5)

        # Test SCLK
        GPIO.output(SCLK, GPIO.HIGH)
        print("SCLK: HIGH")
        time.sleep(0.5)
        GPIO.output(SCLK, GPIO.LOW)
        print("SCLK: LOW")
        time.sleep(0.5)

        # Test SS
        GPIO.output(SS, GPIO.HIGH)
        print("SS: HIGH")
        time.sleep(0.5)
        GPIO.output(SS, GPIO.LOW)
        print("SS: LOW")
        time.sleep(0.5)

finally:
    GPIO.cleanup()

