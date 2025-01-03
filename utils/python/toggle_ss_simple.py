import RPi.GPIO as GPIO
import time

SS_PIN = 8
GPIO.setmode(GPIO.BCM)
GPIO.setup(SS_PIN, GPIO.OUT)

while True:
    GPIO.output(SS_PIN, GPIO.LOW)  # Enable SPI slave
    time.sleep(1)
    GPIO.output(SS_PIN, GPIO.HIGH)  # Disable SPI slave
    time.sleep(1)

